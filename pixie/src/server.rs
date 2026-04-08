use std::{
    env, fs,
    io::{self, BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    sync::Arc,
};

use super::{log_error, log_info, log_warn};

use crate::ThreadPool;

const DEFAULT_WEB_ROOT: &str = "/usr/share/pixie/web";
const INDEX_PAGE: &str = "hello.html";
const NOT_FOUND_PAGE: &str = "404.html";

pub fn run_server(addr: &str, pool_size: usize) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    let pool = ThreadPool::new(pool_size);
    let web_root = Arc::new(resolve_web_root());

    log_info(format_args!(
        "listening on {addr} with {pool_size} workers (web_root={})",
        web_root.display()
    ));

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(err) => {
                log_warn(format_args!("incoming connection failed: {err}"));
                continue;
            }
        };

        let web_root = Arc::clone(&web_root);
        pool.execute(move || {
            if let Err(err) = handle_connection(stream, web_root.as_path()) {
                log_error(format_args!("failed to handle connection: {err}"));
            }
        });
    }

    Ok(())
}

fn resolve_web_root() -> PathBuf {
    if let Ok(path) = env::var("PIXIE_WEB_ROOT") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            let candidate = PathBuf::from(trimmed);
            if candidate.is_dir() {
                return candidate;
            }
            log_warn(format_args!(
                "PIXIE_WEB_ROOT='{}' is not a directory, using fallback",
                candidate.display()
            ));
        }
    }

    let dev_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../web");
    if dev_root.is_dir() {
        return dev_root;
    }

    PathBuf::from(DEFAULT_WEB_ROOT)
}

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

fn resolve_route(request_line: &str, web_root: &Path) -> (&'static str, PathBuf) {
    let not_found = || ("HTTP/1.1 404 NOT FOUND", web_root.join(NOT_FOUND_PAGE));

    if !request_line.starts_with("GET /") {
        return not_found();
    }

    let route = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .trim_start_matches('/')
        .split('?')
        .next()
        .unwrap_or("");

    let file = if route.is_empty() {
        web_root.join(INDEX_PAGE)
    } else {
        web_root.join(format!("{route}.html"))
    };

    if file.is_file() {
        ("HTTP/1.1 200 OK", file)
    } else {
        not_found()
    }
}
