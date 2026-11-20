//! Pixie HTTP/1.x static server.

use std::{
    fs,
    io::{self, BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    sync::Arc,
    time::Duration,
};

use crate::{
    ThreadPool,
    logger::{log_error, log_info},
    router::{resolve_route, resolve_web_root},
};

const MAX_REQUEST_BYTES: usize = 8 * 1024;
const IO_TIMEOUT: Duration = Duration::from_secs(5);

pub fn run_server(addr: &str, pool_size: usize) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    let pool = ThreadPool::new(pool_size);
    let web_root = Arc::new(resolve_web_root());
    log_info(format_args!(
        "listening on {addr} with {pool_size} workers (web_root={})",
        web_root.display()
    ));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let root = Arc::clone(&web_root);
                pool.execute(move || {
                    if let Err(err) = handle_connection(stream, root.as_path()) {
                        log_error(format_args!("failed to handle connection: {err}"));
                    }
                });
            }
            Err(err) => log_error(format_args!("failed to accept connection: {err}")),
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream, web_root: &Path) -> io::Result<()> {
    stream.set_read_timeout(Some(IO_TIMEOUT))?;
    stream.set_write_timeout(Some(IO_TIMEOUT))?;
    let mut reader = BufReader::new(&mut stream);
    let mut request = Vec::new();
    loop {
        if request.len() >= MAX_REQUEST_BYTES {
            return write_response(
                &mut stream,
                "HTTP/1.1 431 Request Header Fields Too Large",
                "Request too large",
                false,
            );
        }
        let mut line = Vec::new();
        let n = (&mut reader)
            .take((MAX_REQUEST_BYTES - request.len() + 1) as u64)
            .read_until(b'\n', &mut line)?;
        if n == 0 {
            return Ok(());
        }
        request.extend_from_slice(&line);
        if request.ends_with(b"\r\n\r\n") || request.ends_with(b"\n\n") {
            break;
        }
    }
    drop(reader);
    if request.len() > MAX_REQUEST_BYTES {
        return write_response(
            &mut stream,
            "HTTP/1.1 431 Request Header Fields Too Large",
            "Request too large",
            false,
        );
    }
    let Ok(raw) = std::str::from_utf8(&request) else {
        return write_response(
            &mut stream,
            "HTTP/1.1 400 Bad Request",
            "Bad request",
            false,
        );
    };
    let mut lines = raw.split("\r\n");
    let Some(start) = lines.next() else {
        return bad_request(&mut stream);
    };
    let mut parts = start.split_ascii_whitespace();
    let (Some(method), Some(target), Some(version)) = (parts.next(), parts.next(), parts.next())
    else {
        return bad_request(&mut stream);
    };
    if parts.next().is_some() || !target.starts_with('/') {
        return bad_request(&mut stream);
    }
    let http10 = version == "HTTP/1.0";
    if !http10 && version != "HTTP/1.1" {
        return write_response(
            &mut stream,
            "HTTP/1.1 505 HTTP Version Not Supported",
            "Version not supported",
            false,
        );
    }
    let headers = lines.filter(|l| !l.is_empty()).collect::<Vec<_>>();
    if headers
        .iter()
        .any(|h| h.starts_with(' ') || h.starts_with('\t') || !h.contains(':'))
    {
        return bad_request(&mut stream);
    }
    if !http10
        && !headers.iter().any(|h| {
            h.split_once(':')
                .is_some_and(|(k, v)| k.eq_ignore_ascii_case("host") && !v.trim().is_empty())
        })
    {
        return bad_request(&mut stream);
    }
    if headers.iter().any(|h| {
        h.split_once(':').is_some_and(|(k, v)| {
            k.eq_ignore_ascii_case("transfer-encoding")
                || (k.eq_ignore_ascii_case("content-length") && v.trim() != "0")
        })
    }) {
        return write_response(
            &mut stream,
            "HTTP/1.1 400 Bad Request",
            "Request body not supported",
            false,
        );
    }
    if method != "GET" && method != "HEAD" {
        return write_response(
            &mut stream,
            "HTTP/1.1 405 Method Not Allowed",
            "Method not allowed",
            false,
        );
    }
    let request_line = format!("GET {target} HTTP/1.1");
    let (status, filename) = resolve_route(&request_line, web_root);
    let body = match fs::read_to_string(&filename) {
        Ok(body) => body,
        Err(_err) if status.contains("404") => {
            return write_response(
                &mut stream,
                "HTTP/1.1 404 NOT FOUND",
                "Not Found",
                method == "HEAD",
            );
        }
        Err(_) => {
            return write_response(
                &mut stream,
                "HTTP/1.1 500 Internal Server Error",
                "Internal Server Error",
                method == "HEAD",
            );
        }
    };
    write_response(&mut stream, status, &body, method == "HEAD")
}

fn bad_request(stream: &mut TcpStream) -> io::Result<()> {
    write_response(stream, "HTTP/1.1 400 Bad Request", "Bad request", false)
}

fn write_response(stream: &mut TcpStream, status: &str, body: &str, head: bool) -> io::Result<()> {
    write!(
        stream,
        "{status}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    if !head {
        stream.write_all(body.as_bytes())?;
    }
    Ok(())
}
