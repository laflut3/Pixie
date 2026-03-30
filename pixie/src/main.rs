use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    path::Path,

};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let status_line = "HTTP/1.1 200 OK";

    let file_name = request_line
            .split_whitespace()
            .nth(1)
            .unwrap_or("")
            .trim_start_matches('/')
            .split('?')
            .next()
            .unwrap_or("");

    let html_path = if file_name.is_empty() {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../web/hello.html")
    } else {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../web")
            .join(format!("{file_name}.html"))
    };

    let contents = fs::read_to_string(html_path).unwrap();
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
}
