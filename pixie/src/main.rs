use std::{
    env, io,
    process::{self, Command},
};

use pixie::run_server;

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);

    match args.next().as_deref() {
        None | Some("serve") => run_server("127.0.0.1:80", 4),
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
