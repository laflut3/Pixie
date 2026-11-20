use std::{io, process};

use pixie::{log_error, runtime_config, server::run_server_with_protocols};

/// Binary entry point.
///
/// The binary only starts the HTTP server.
/// Exit code `2` on initial configuration errors.
fn main() -> io::Result<()> {
    let config = match runtime_config() {
        Ok(config) => config,
        Err(err) => {
            log_error(format_args!("{err}"));
            process::exit(2);
        }
    };

    run_server_with_protocols(&config.addr, config.workers, config.tls, config.http3)
}
