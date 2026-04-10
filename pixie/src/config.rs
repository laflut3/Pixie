use serde::Deserialize;
use std::{env, fs, io, path::Path};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 80;
const DEFAULT_THREADS: usize = 4;
const LOCAL_CONFIG_PATH: &str = "config-pixie.yml";
const SYSTEM_CONFIG_PATH: &str = "/etc/pixie/config-pixie.yml";
const CONFIG_ENV_VAR: &str = "PIXIE_CONFIG";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    pub addr: String,
    pub workers: usize,
}

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

// Builds the fallback bind address from default host and port.
fn default_addr() -> String {
    format!("{DEFAULT_HOST}:{DEFAULT_PORT}")
}

// Reads an optional config path override from PIXIE_CONFIG.
fn env_config_path() -> Option<String> {
    env_value(CONFIG_ENV_VAR)
}

// Resolves runtime config with precedence: file values, then hardcoded defaults.
pub fn runtime_config() -> io::Result<RuntimeConfig> {
    let file_config = load_file_config()?;

    let addr = file_addr(file_config.as_ref()).unwrap_or_else(default_addr);

    let workers = file_config
        .and_then(|cfg| cfg.workers)
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_THREADS);

    Ok(RuntimeConfig { addr, workers })
}

// Loads configuration from file, trying env override first, then system/local defaults.
fn load_file_config() -> io::Result<Option<FileConfig>> {
    if let Some(path) = env_config_path() {
        return read_config_if_exists(&path);
    }

    for path in [SYSTEM_CONFIG_PATH, LOCAL_CONFIG_PATH] {
        if let Some(cfg) = read_config_if_exists(path)? {
            return Ok(Some(cfg));
        }
    }

    Ok(None)
}

// Reads and parses a YAML file only when it exists on disk.
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

// Derives an address from file config using `addr` first, else `host` + `port`.
fn file_addr(config: Option<&FileConfig>) -> Option<String> {
    let config = config?;

    config.addr.clone().or_else(|| {
        (config.host.is_some() || config.port.is_some()).then(|| {
            let host = config.host.as_deref().unwrap_or(DEFAULT_HOST);
            let port = config.port.unwrap_or(DEFAULT_PORT);
            format!("{host}:{port}")
        })
    })
}

// Returns a non-empty environment variable value without extra normalization.
fn env_value(key: &str) -> Option<String> {
    env::var(key)
        .ok()
        .filter(|value| !value.is_empty())
}
