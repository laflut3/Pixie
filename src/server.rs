//! Pixie static HTTP/1.x and HTTP/2 server.

use std::{io, net::SocketAddr, sync::Arc, time::Duration};

use bytes::Bytes;
use http_body_util::Full;
use hyper::{Request, Response, StatusCode, body::Incoming, service::service_fn};
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto::Builder,
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpListener,
    time::timeout,
};

use crate::{
    TlsConfig,
    logger::{log_error, log_info},
    router::read_response,
    sites::Sites,
    tls::load_server_config,
};

pub fn run_server(addr: &str, pool_size: usize) -> io::Result<()> {
    run_server_with_tls(addr, pool_size, None)
}

/// Starts a Tokio based HTTP/1.x and HTTP/2 server with optional TLS/mTLS.
pub fn run_server_with_tls(addr: &str, pool_size: usize, tls: Option<TlsConfig>) -> io::Result<()> {
    run_server_with_protocols(addr, pool_size, tls, None)
}

pub fn run_server_with_protocols(
    addr: &str,
    pool_size: usize,
    tls: Option<TlsConfig>,
    http3_addr: Option<String>,
) -> io::Result<()> {
    let addr = addr.to_owned();
    let mut tls_config = tls.as_ref().map(load_server_config).transpose()?;
    if let Some(config) = &mut tls_config {
        Arc::get_mut(config)
            .expect("new TLS config is uniquely owned")
            .alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    }
    let sites = Sites::from_env();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(pool_size.max(1))
        .enable_all()
        .build()
        .map_err(io::Error::other)?;
    runtime.block_on(async move {
        if let Some(http3_addr) = http3_addr {
            let mut h3_tls = (**tls_config.as_ref().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "HTTP/3 requires TLS")
            })?)
            .clone();
            h3_tls.alpn_protocols = vec![b"h3".to_vec()];
            let crypto = quinn::crypto::rustls::QuicServerConfig::try_from(h3_tls)
                .map_err(io::Error::other)?;
            let endpoint = quinn::Endpoint::server(
                quinn::ServerConfig::with_crypto(Arc::new(crypto)),
                http3_addr.parse().map_err(io::Error::other)?,
            )?;
            let root = sites.clone();
            tokio::spawn(async move {
                log_info(format_args!(
                    "HTTP/3 listening on {http3_addr} (experimental)"
                ));
                while let Some(connecting) = endpoint.accept().await {
                    let root = root.clone();
                    tokio::spawn(async move {
                        if let Ok(connection) = connecting.await {
                            let mut h3 = match h3::server::builder()
                                .build(h3_quinn::Connection::new(connection))
                                .await
                            {
                                Ok(h3) => h3,
                                Err(_) => return,
                            };
                            while let Ok(Some(resolver)) = h3.accept().await {
                                let root = root.clone();
                                tokio::spawn(async move {
                                    if let Ok((request, mut stream)) =
                                        resolver.resolve_request().await
                                    {
                                        let result = read_response(
                                            request.method().as_str(),
                                            request.uri().path(),
                                            request.uri().authority().map(|a| a.as_str()).or_else(
                                                || {
                                                    request
                                                        .headers()
                                                        .get(hyper::header::HOST)
                                                        .and_then(|h| h.to_str().ok())
                                                },
                                            ),
                                            &root,
                                        );
                                        let length = result.body.len();
                                        let body = if request.method() == hyper::Method::HEAD {
                                            Vec::new()
                                        } else {
                                            result.body
                                        };
                                        let response = hyper::Response::builder()
                                            .status(result.status)
                                            .header("content-type", result.content_type)
                                            .header("content-length", length)
                                            .header("x-content-type-options", "nosniff")
                                            .body(())
                                            .unwrap();
                                        if stream.send_response(response).await.is_ok() {
                                            let _ = stream.send_data(Bytes::from(body)).await;
                                            let _ = stream.finish().await;
                                        }
                                    }
                                });
                            }
                        }
                    });
                }
            });
        }
        let listener = TcpListener::bind(&addr).await?;
        log_info(format_args!(
            "listening on {addr} with {pool_size} workers, http/1 + http/2, tls={} (web_root={})",
            tls_config.is_some(),
            sites.directory().display()
        ));
        loop {
            let (stream, peer) = listener.accept().await?;
            let root = sites.clone();
            let tls = tls_config.clone();
            tokio::spawn(async move {
                if let Err(err) = serve_connection(stream, peer, root, tls).await {
                    log_error(format_args!("failed to handle connection: {err}"));
                }
            });
        }
    })
}

async fn serve_connection(
    stream: tokio::net::TcpStream,
    _peer: SocketAddr,
    root: Sites,
    tls: Option<Arc<rustls::ServerConfig>>,
) -> io::Result<()> {
    stream.set_nodelay(true)?;
    let io: TokioIo<Box<dyn AsyncIo>> = if let Some(config) = tls {
        let acceptor = tokio_rustls::TlsAcceptor::from(config);
        let tls_stream = timeout(Duration::from_secs(5), acceptor.accept(stream))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "TLS handshake timed out"))??;
        TokioIo::new(Box::new(tls_stream))
    } else {
        TokioIo::new(Box::new(stream))
    };
    let mut builder = Builder::new(TokioExecutor::new());
    builder.http1().keep_alive(false);
    builder
        .serve_connection(io, service_fn(move |req| respond(req, root.clone())))
        .await
        .map_err(io::Error::other)
}

trait AsyncIo: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> AsyncIo for T {}

async fn respond(req: Request<Incoming>, root: Sites) -> Result<Response<Full<Bytes>>, io::Error> {
    let method = req.method().clone();
    let http_version = req.version();
    if method != hyper::Method::GET && method != hyper::Method::HEAD {
        let mut response = response(
            StatusCode::METHOD_NOT_ALLOWED,
            "Method not allowed",
            method == hyper::Method::HEAD,
        );
        close_http1(&mut response, http_version);
        return Ok(response);
    }
    let host = req.uri().authority().map(|a| a.as_str()).or_else(|| {
        req.headers()
            .get(hyper::header::HOST)
            .and_then(|h| h.to_str().ok())
    });
    let result = read_response(method.as_str(), req.uri().path(), host, &root);
    let body = result.body;
    let status = StatusCode::from_u16(result.status).unwrap();
    let length = body.len();
    let mut response = Response::new(Full::new(if method == hyper::Method::HEAD {
        Bytes::new()
    } else {
        Bytes::from(body)
    }));
    *response.status_mut() = status;
    response.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        result.content_type.parse().unwrap(),
    );
    response.headers_mut().insert(
        hyper::header::CONTENT_LENGTH,
        length.to_string().parse().unwrap(),
    );
    response
        .headers_mut()
        .insert("x-content-type-options", "nosniff".parse().unwrap());
    close_http1(&mut response, http_version);
    Ok(response)
}

fn close_http1(response: &mut Response<Full<Bytes>>, version: hyper::Version) {
    if version != hyper::Version::HTTP_2 {
        response
            .headers_mut()
            .insert(hyper::header::CONNECTION, "close".parse().unwrap());
    }
}

fn response(status: StatusCode, body: &'static str, head: bool) -> Response<Full<Bytes>> {
    let mut response = Response::new(Full::new(if head {
        Bytes::new()
    } else {
        Bytes::from_static(body.as_bytes())
    }));
    *response.status_mut() = status;
    response.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        "text/plain; charset=utf-8".parse().unwrap(),
    );
    response.headers_mut().insert(
        hyper::header::CONTENT_LENGTH,
        body.len().to_string().parse().unwrap(),
    );
    if status == StatusCode::METHOD_NOT_ALLOWED {
        response
            .headers_mut()
            .insert(hyper::header::ALLOW, "GET, HEAD".parse().unwrap());
    }
    response
}
