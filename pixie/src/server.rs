//! Serveur TCP/HTTP de Pixie.
//!
//! Le serveur accepte des connexions TCP, lit la première ligne HTTP,
//! résout une page statique avec le routeur, puis renvoie une réponse HTML.

use std::{
    fs,
    io::{self, BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
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

    for stream in listener.incoming().flatten() {
        dispatch_stream(stream, &pool, web_root.as_path());
    }

    Ok(())
}

/// Délègue une connexion acceptée au thread-pool.
fn dispatch_stream(stream: TcpStream, pool: &ThreadPool, web_root: &Path) {
    let web_root = PathBuf::from(web_root);

    pool.execute(move || {
        if let Err(err) = handle_connection(stream, web_root.as_path()) {
            log_error(format_args!("failed to handle connection: {err}"));
        }
    });
}

/// Lit la request-line HTTP, résout la page cible puis écrit la réponse.
fn handle_connection(mut stream: TcpStream, web_root: &Path) -> io::Result<()> {
    let buf_reader = BufReader::new(&stream);
    let request_line = match buf_reader.lines().next().transpose()? {
        Some(line) => line,
        None => return Ok(()),
    };

    let (status_line, filename) = resolve_route(&request_line, web_root);
    let contents = fs::read_to_string(filename)?;
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes())?;
    Ok(())
}
