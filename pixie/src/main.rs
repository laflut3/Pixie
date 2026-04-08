use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
};

use pixie::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn add_path(request_line: String) -> PathBuf {

    let path = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("")
        .trim_start_matches('/')
        .split('?')
        .next()
        .unwrap_or("");
    
    let html_path = if path.is_empty() {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../web/hello.html")
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../web")
            .join(format!("{path}.html"))
    };

    return html_path
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let (mut status_line, mut filename) = if request_line.starts_with("GET /") {
        let status = "HTTP/1.1 200 OK";
        let html_path = add_path(request_line);
        (status, html_path)
    } else {
        let error_page = Path::new(env!("CARGO_MANIFEST_DIR")).join("../web/404.html");
        ("HTTP/1.1 404 NOT FOUND", error_page)
    };

    if !filename.exists() {
        status_line = "HTTP/1.1 404 NOT FOUND";
        filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("../web/404.html");
    }

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
