use std::{
    env, io,
    process::{self, Command},
};

use pixie::run_server;

const DEFAULT_ADDR: &str = "127.0.0.1:80";
const DEFAULT_THREADS: usize = 4;

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        None | Some("serve") => {
            let addr = server_addr();
            let pool_size = pool_size();
            run_server(&addr, pool_size)
        }
        Some("log") | Some("logs") => {
            let extra_args: Vec<String> = args.collect();
            show_logs(&extra_args)
        }
        Some(_) => {
            eprintln!("Usage:");
            eprintln!("  pixie serve");
            eprintln!("  pixie log [journalctl options]");
            process::exit(2);
        }
    }
}

fn server_addr() -> String {
    match env::var("PIXIE_ADDR") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => DEFAULT_ADDR.to_string(),
    }
}

fn pool_size() -> usize {
    match env::var("PIXIE_THREADS") {
        Ok(value) => match value.parse::<usize>() {
            Ok(size) if size > 0 => size,
            _ => {
                eprintln!(
                    "Invalid PIXIE_THREADS='{value}', using default {}",
                    DEFAULT_THREADS
                );
                DEFAULT_THREADS
            }
        },
        Err(_) => DEFAULT_THREADS,
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
        eprintln!("journalctl exited with status: {status}");
    }

    Ok(())
}
