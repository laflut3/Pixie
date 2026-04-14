//! Journalisation minimale de Pixie vers la sortie d'erreur.

use std::fmt;

/// Écrit un message de niveau `info` sur stderr.
pub fn log_info(args: fmt::Arguments<'_>) {
    eprintln!("[pixie][info] {args}");
}

/// Écrit un message de niveau `warn` sur stderr.
pub fn log_warn(args: fmt::Arguments<'_>) {
    eprintln!("[pixie][warn] {args}");
}

/// Écrit un message de niveau `error` sur stderr.
pub fn log_error(args: fmt::Arguments<'_>) {
    eprintln!("[pixie][error] {args}");
}
