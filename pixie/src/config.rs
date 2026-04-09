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
    #[serde(default)]
    workers: Option<usize>,
}

pub fn runtime_config() -> io::Result<RuntimeConfig> {
    let file_config = load_file_config()?;

    let addr = file_addr(file_config.as_ref())
        .or_else(env_addr)
        .unwrap_or_else(default_addr);

    let workers = file_config
        .and_then(|cfg| cfg.workers)
        .or_else(env_workers)
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_THREADS);

    Ok(RuntimeConfig { addr, workers })
}

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

fn env_config_path() -> Option<String> {
    env::var(CONFIG_ENV_VAR)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

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

fn file_addr(config: Option<&FileConfig>) -> Option<String> {
    let config = config?;

    if let Some(addr) = clean(&config.addr) {
        return Some(addr.to_string());
    }

    if config.host.is_some() || config.port.is_some() {
        let host = clean(&config.host).unwrap_or(DEFAULT_HOST);
        let port = config.port.unwrap_or(DEFAULT_PORT);
        return Some(format!("{host}:{port}"));
    }

    None
}

fn env_addr() -> Option<String> {
    env::var("PIXIE_ADDR")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn env_workers() -> Option<usize> {
    env::var("PIXIE_THREADS")
        .ok()
        .and_then(|value| value.trim().parse::<usize>().ok())
}

fn default_addr() -> String {
    format!("{DEFAULT_HOST}:{DEFAULT_PORT}")
}

fn clean(value: &Option<String>) -> Option<&str> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|trimmed| !trimmed.is_empty())
}
