use std::{
    fmt
};

pub fn log_info(args: fmt::Arguments<'_>) {
    eprintln!("[pixie][info] {args}");
}

pub fn log_warn(args: fmt::Arguments<'_>) {
    eprintln!("[pixie][warn] {args}");
}

pub fn log_error(args: fmt::Arguments<'_>) {
    eprintln!("[pixie][error] {args}");
}