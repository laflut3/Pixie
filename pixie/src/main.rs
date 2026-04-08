use std::{env, io, process};

use pixie::{log_error, log_info, pool_size, run_server, server_addr, show_logs};

const USAGE: &str = "Usage:\n  pixie serve\n  pixie log [journalctl options]";

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        None | Some("serve") => {
            let addr = server_addr();

            let worker_count = match pool_size() {
                Ok(size) => size,
                Err(err) => {
                    log_error(format_args!("{err}"));
                    process::exit(2);
                }
            };

            run_server(&addr, worker_count)
        }
        Some("log") | Some("logs") => {
            let extra_args: Vec<String> = args.collect();
            show_logs(&extra_args)
        }
        Some(command) => {
            log_error(format_args!("unknown command: {command}"));
            log_info(format_args!("{USAGE}"));
            process::exit(2);
        }
    }
}
