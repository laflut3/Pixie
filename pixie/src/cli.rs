use std::{io, process::Command};

use crate::log_error;

pub fn show_logs(extra_args: &[String]) -> io::Result<()> {
    let mut command = Command::new("journalctl");
    command.arg("-u").arg("pixie.service");

    if extra_args.is_empty() {
        command.arg("-n").arg("100").arg("--no-pager");
    } else {
        command.args(extra_args);
    }

    let status = command.status()?;
    if !status.success() {
        log_error(format_args!("journalctl exited with status: {status}"));
    }

    Ok(())
}
