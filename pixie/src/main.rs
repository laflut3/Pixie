use std::{env, io, process};

use pixie::{log_error, log_info, run_server, runtime_config};

const USAGE: &str = "Usage:\n  pixie serve";

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        None | Some("serve") => {
            let config = match runtime_config() {
                Ok(config) => config,
                Err(err) => {
                    log_error(format_args!("{err}"));
                    process::exit(2);
                }
            };

            run_server(&config.addr, config.workers)
        }
        Some(command) => {
            log_error(format_args!("unknown command: {command}"));
            log_info(format_args!("{USAGE}"));
            process::exit(2);
        }
    }
}
