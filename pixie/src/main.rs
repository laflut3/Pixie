use std::{
    env, io,
    process::{self, Command},
};

use pixie::run_server;

const DEFAULT_ADDR: &str = "127.0.0.1:80";
const DEFAULT_THREADS: usize = 4;
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

fn server_addr() -> String {
    env::var("PIXIE_ADDR")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_ADDR.to_string())
}

fn pool_size() -> io::Result<usize> {
    match env::var("PIXIE_THREADS") {
        Ok(value) => {
            let trimmed = value.trim();

            if trimmed.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "PIXIE_THREADS is set but empty",
                ));
            }

            let size = trimmed.parse::<usize>().map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("PIXIE_THREADS='{value}' is not a valid positive integer"),
                )
            })?;

            if size == 0 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "PIXIE_THREADS must be greater than 0",
                ));
            }

            Ok(size)
        }
        Err(env::VarError::NotPresent) => Ok(DEFAULT_THREADS),
        Err(env::VarError::NotUnicode(_)) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "PIXIE_THREADS contains invalid UTF-8",
        )),
    }
}

fn show_logs(extra_args: &[String]) -> io::Result<()> {
    let mut command = Command::new("journalctl");
    command.arg("-u").arg("pixie.service");

    if extra_args.is_empty() {
        command.arg("-n").arg("100").arg("--no-pager");
    } else {
        command.args(extra_args);
    }

    let status = command.status()?;
    if !status.success() {
        eprintln!("[pixie][error] journalctl exited with status: {status}");
    }

    Ok(())
}
