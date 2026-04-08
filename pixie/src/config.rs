use std::{env, io};

const DEFAULT_ADDR: &str = "127.0.0.1:80";
const DEFAULT_THREADS: usize = 4;

pub fn server_addr() -> String {
    env::var("PIXIE_ADDR")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| DEFAULT_ADDR.to_string())
}

pub fn pool_size() -> io::Result<usize> {
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
