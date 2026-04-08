use std::{env, io, process};

use pixie::{pool_size, run_server, server_addr, show_logs};

const USAGE: &str = "Usage:\n  pixie serve\n  pixie log [journalctl options]";

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        None | Some("serve") => {
            let addr = server_addr();

            let pool_size = match pool_size() {
                Ok(size) => size,
                Err(err) => {
                    eprintln!("[pixie][error] {err}");
                    process::exit(2);
                }
            };

            run_server(&addr, pool_size)
        }
        Some("log") | Some("logs") => {
            let extra_args: Vec<String> = args.collect();
            show_logs(&extra_args)
        }
        Some(command) => {
            eprintln!("[pixie][error] unknown command: {command}");
            eprintln!("{USAGE}");
            process::exit(2);
        }
    }
}
