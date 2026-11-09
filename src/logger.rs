//! Minimal logging from Pixie to standard error.

use std::fmt;

/// Writes a message at the `info` level to standard error.
pub fn log_info(args: fmt::Arguments<'_>) {
    eprintln!("[pixie][info] {args}");
}

/// Writes a message at the `warn` level to standard error.
pub fn log_warn(args: fmt::Arguments<'_>) {
    eprintln!("[pixie][warn] {args}");
}

/// Writes a message at the `error` level to standard error.
pub fn log_error(args: fmt::Arguments<'_>) {
    eprintln!("[pixie][error] {args}");
}
