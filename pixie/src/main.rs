use std::{env, io, process};

use pixie::{RuntimeConfig, log_error, log_info, run_server, runtime_config};

const USAGE: &str = "Usage:\n  pixie serve";

/// Commandes CLI supportées par Pixie.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    /// Démarre le serveur HTTP.
    Serve,
}

/// Point d'entrée binaire.
///
/// Codes de sortie:
/// - `0`: exécution normale
/// - `2`: erreur de configuration ou commande invalide
fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);

    let command = match parse_command(&mut args) {
        Ok(command) => command,
        Err(command) => {
            log_error(format_args!("unknown command: {command}"));
            log_info(format_args!("{USAGE}"));
            process::exit(2);
        }
    };

    if let Err(err) = execute(command) {
        log_error(format_args!("{err}"));
        process::exit(2);
    }

    Ok(())
}

/// Parse la première commande CLI et applique la valeur par défaut `serve`.
fn parse_command(args: &mut impl Iterator<Item = String>) -> Result<Command, String> {
    match args.next().as_deref() {
        None | Some("serve") => Ok(Command::Serve),
        Some(command) => Err(command.to_string()),
    }
}

/// Exécute la commande demandée.
fn execute(command: Command) -> io::Result<()> {
    match command {
        Command::Serve => {
            let config: RuntimeConfig = runtime_config()?;
            run_server(&config.addr, config.workers)
        }
    }
}
