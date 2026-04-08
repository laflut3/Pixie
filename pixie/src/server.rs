use std::{
    env,
    fs,
    io::{self, BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
};

use crate::ThreadPool;

pub fn run_server(addr: &str, pool_size: usize) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;
    let pool = ThreadPool::new(pool_size);
    eprintln!("Pixie listening on {addr} with {pool_size} workers");

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(err) => {
                eprintln!("Connection failed: {err}");
                continue;
            }
        };

        pool.execute(move || {
            if let Err(err) = handle_connection(stream) {
                eprintln!("Failed to handle connection: {err}");
            }
        });
    }

    Ok(())
}

fn web_root() -> PathBuf {
    if let Ok(path) = env::var("PIXIE_WEB_ROOT") {
        let candidate = PathBuf::from(path);
        if candidate.exists() {
            return candidate;
        }
    }

    let dev_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../web");
    if dev_root.exists() {
        return dev_root;
    }

    PathBuf::from("/usr/share/pixie/web")
}

fn not_found_path() -> PathBuf {
    web_root().join("404.html")
}

fn add_path(request_line: &str) -> PathBuf {
    let path = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("")
        .trim_start_matches('/')
        .split('?')
        .next()
        .unwrap_or("");

    let root = web_root();
    let html_path = if path.is_empty() {
        root.join("hello.html")
    } else {
        root.join(format!("{path}.html"))
    };

    html_path
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let buf_reader = BufReader::new(&stream);
    let request_line = match buf_reader.lines().next().transpose()? {
        Some(line) => line,
        None => return Ok(()),
    };

    let (mut status_line, mut filename) = if request_line.starts_with("GET /") {
        let status = "HTTP/1.1 200 OK";
        let html_path = add_path(&request_line);
        (status, html_path)
    } else {
        ("HTTP/1.1 404 NOT FOUND", not_found_path())
    };

    if !filename.exists() {
        status_line = "HTTP/1.1 404 NOT FOUND";
        filename = not_found_path();
    }

    let contents = fs::read_to_string(filename)?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    Ok(())
}
