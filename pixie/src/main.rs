use std::io;

use pixie::run_server;

fn main() -> io::Result<()> {
    run_server("127.0.0.1:80", 4)
}
