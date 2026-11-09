//! Runtime configuration for Pixie.
//!
//! Configuration lookup order:
//! 1. file set by `PIXIE_CONFIG` (when non-empty)
//! 2. `config-pixie.yml` in the current directory
//! 3. `/etc/pixie/config-pixie.yml`
//! 4. compiled defaults

use serde::Deserialize;
use std::{env, fs, io, path::Path};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 80;
const DEFAULT_THREADS: usize = 4;
const LOCAL_CONFIG_PATH: &str = "config-pixie.yml";
const SYSTEM_CONFIG_PATH: &str = "/etc/pixie/config-pixie.yml";
const CONFIG_ENV_VAR: &str = "PIXIE_CONFIG";

/// Final configuration used by the server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    /// HTTP listen address in `host:port` format.
    pub addr: String,
    /// Number of worker threads.
    pub workers: usize,
}

/// Partial YAML representation before normalization.
#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
struct FileConfig {
    #[serde(default, alias = "address")]
    addr: Option<String>,
    #[serde(default)]
    host: Option<String>,
    #[serde(default)]
    port: Option<u16>,
    #[serde(default, alias = "nb_worker")]
    workers: Option<usize>,
}

/// Builds the default address (`127.0.0.1:80`).
fn default_addr() -> String {
    format!("{DEFAULT_HOST}:{DEFAULT_PORT}")
}

/// Reads `PIXIE_CONFIG` and returns its value only when it is non-empty.
fn env_config_path() -> Option<String> {
    env::var(CONFIG_ENV_VAR)
        .ok()
        .filter(|value| !value.is_empty())
}

/// Resolves runtime configuration using file priority,
/// then falls back to defaults when necessary.
pub fn runtime_config() -> io::Result<RuntimeConfig> {
    let file_config = load_file_config()?;

    let addr = file_addr(file_config.as_ref()).unwrap_or_else(default_addr);

    let workers = match file_config.and_then(|cfg| cfg.workers) {
        Some(value) if value > 0 => value,
        _ => DEFAULT_THREADS,
    };

    Ok(RuntimeConfig { addr, workers })
}

/// Loads the first configuration file found in priority order.
fn load_file_config() -> io::Result<Option<FileConfig>> {
    if let Some(path) = env_config_path() {
        return read_config_if_exists(&path);
    }

    for path in [LOCAL_CONFIG_PATH, SYSTEM_CONFIG_PATH] {
        if let Some(config) = read_config_if_exists(path)? {
            return Ok(Some(config));
        }
    }

    Ok(None)
}

/// Reads and parses a YAML file only when it exists and is a regular file.
fn read_config_if_exists(path: &str) -> io::Result<Option<FileConfig>> {
    let path = Path::new(path);

    if !path.is_file() {
        return Ok(None);
    }

    let raw = fs::read_to_string(path)?;

    let config = serde_yaml::from_str::<FileConfig>(&raw).map_err(|err| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid YAML in {}: {err}", path.display()),
        )
    })?;

    Ok(Some(config))
}

/// Computes the address from file configuration.
///
/// Rules:
/// - `addr` takes precedence
/// - otherwise `host` and `port` are combined with defaults for missing values
/// - otherwise `None`
fn file_addr(config: Option<&FileConfig>) -> Option<String> {
    let config = config?;

    if let Some(addr) = config.addr.clone() {
        return Some(addr);
    }

    if config.host.is_none() && config.port.is_none() {
        return None;
    }

    let host = config.host.as_deref().unwrap_or(DEFAULT_HOST);
    let port = config.port.unwrap_or(DEFAULT_PORT);
    Some(format!("{host}:{port}"))
}
