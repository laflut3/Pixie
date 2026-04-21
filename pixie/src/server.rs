//! Serveur TCP/HTTP de Pixie.
//!
//! Le serveur accepte des connexions TCP, lit la première ligne HTTP,
//! résout une page statique avec le routeur, puis renvoie une réponse HTML.

use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    sync::Arc,
};

use crate::{
    ThreadPool,
    logger::{log_error, log_info},
    router::{resolve_route, resolve_web_root},
};

/// Démarre le serveur HTTP sur `addr` avec `pool_size` workers.
///
/// La fonction boucle sur les connexions entrantes tant que le listener reste ouvert.
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
                let web_root = Arc::clone(&web_root);
                pool.execute(move || {
                    if let Err(err) = handle_connection(stream, web_root.as_path()) {
                        log_error(format_args!("failed to handle connection: {err}"));
                    }
                });
            }
            Err(err) => log_error(format_args!("failed to accept connection: {err}")),
        }
    }

    Ok(())
}

/// Lit la request-line HTTP, résout la page cible puis écrit la réponse.
fn handle_connection(mut stream: TcpStream, web_root: &Path) -> io::Result<()> {
    let mut request_line = String::new();
    
    if BufReader::new(&stream).read_line(&mut request_line)? == 0 {
        return Ok(());
    }

    let request_line = request_line.trim_end_matches(['\r', '\n']);

    let (status_line, filename) = resolve_route(request_line, web_root);
    let contents = fs::read_to_string(&filename)?;
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes())?;
    Ok(())
}
