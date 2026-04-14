//! Gestion de la configuration runtime de Pixie.
//!
//! Priorité de résolution:
//! 1. fichier pointé par `PIXIE_CONFIG` (si non vide)
//! 2. `config-pixie.yml` dans le répertoire courant
//! 3. `/etc/pixie/config-pixie.yml`
//! 4. valeurs par défaut compilées

use serde::Deserialize;
use std::{env, fs, io, path::Path};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 80;
const DEFAULT_THREADS: usize = 4;
const LOCAL_CONFIG_PATH: &str = "config-pixie.yml";
const SYSTEM_CONFIG_PATH: &str = "/etc/pixie/config-pixie.yml";
const CONFIG_ENV_VAR: &str = "PIXIE_CONFIG";

/// Configuration finale utilisée par le serveur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeConfig {
    /// Adresse d'écoute HTTP, au format `hote:port`.
    pub addr: String,
    /// Nombre de workers du thread-pool.
    pub workers: usize,
}

/// Représentation partielle du YAML avant normalisation.
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

/// Construit l'adresse par défaut (`127.0.0.1:80`).
fn default_addr() -> String {
    format!("{DEFAULT_HOST}:{DEFAULT_PORT}")
}

/// Lit `PIXIE_CONFIG` et renvoie sa valeur uniquement si elle est non vide.
fn env_config_path() -> Option<String> {
    env::var(CONFIG_ENV_VAR)
        .ok()
        .filter(|value| !value.is_empty())
}

/// Résout la configuration runtime en appliquant les priorités de fichiers,
/// puis les valeurs par défaut si nécessaire.
pub fn runtime_config() -> io::Result<RuntimeConfig> {
    let file_config = load_file_config()?;

    let addr = file_addr(file_config.as_ref()).unwrap_or_else(default_addr);

    let workers = file_config
        .and_then(|cfg| cfg.workers)
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_THREADS);

    Ok(RuntimeConfig { addr, workers })
}

/// Charge le premier fichier de configuration trouvé selon l'ordre de priorité.
fn load_file_config() -> io::Result<Option<FileConfig>> {
    if let Some(path) = env_config_path() {
        return read_config_if_exists(&path);
    }

    if let Some(cfg) = read_config_if_exists(LOCAL_CONFIG_PATH)? {
        return Ok(Some(cfg));
    }

    read_config_if_exists(SYSTEM_CONFIG_PATH)
}

/// Lit et parse un fichier YAML uniquement s'il existe et s'il s'agit d'un fichier.
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

/// Calcule l'adresse depuis la config fichier.
///
/// Règles:
/// - `addr` gagne toujours
/// - sinon `host`/`port` sont combinés avec les valeurs par défaut manquantes
/// - sinon `None`
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
