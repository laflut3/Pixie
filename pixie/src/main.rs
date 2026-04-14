use std::{io, process};

use pixie::{log_error, run_server, runtime_config};

/// Point d'entree binaire.
///
/// Le binaire lance uniquement le serveur HTTP.
/// Code de sortie `2` en cas d'erreur de configuration initiale.
fn main() -> io::Result<()> {
    let config = match runtime_config() {
        Ok(config) => config,
        Err(err) => {
            log_error(format_args!("{err}"));
            process::exit(2);
        }
    };

    run_server(&config.addr, config.workers)
}
