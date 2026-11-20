mod common;

use common::{EnvGuard, TempFile, env_lock, unique_path};
use pixie::runtime_config;
use std::fs;

/// Verifies the complete fallback when no configuration file is available.
#[test]
fn runtime_config_returns_default_values() {
    let _lock = env_lock();
    let missing = unique_path("pixie-missing-config", "yml");
    let _config = EnvGuard::set(
        "PIXIE_CONFIG",
        missing.to_str().expect("chemin temporaire non UTF-8"),
    );

    let config = runtime_config().expect("configuration runtime invalide");
    assert_eq!(config.addr, "127.0.0.1:80");
    assert_eq!(config.workers, 4);
}

/// Verifies loading a YAML file through `PIXIE_CONFIG`.
#[test]
fn runtime_config_reads_file_from_pixie_config() {
    let _lock = env_lock();
    let config_file = TempFile::new("pixie-config", "host: 0.0.0.0\nport: 9090\nworkers: 8\n");
    let _config = EnvGuard::set("PIXIE_CONFIG", config_file.as_str());

    let config = runtime_config().expect("configuration runtime invalide");
    assert_eq!(config.addr, "0.0.0.0:9090");
    assert_eq!(config.workers, 8);
}

/// Verifies that a TOML file is selected from its extension.
#[test]
fn runtime_config_reads_toml_file() {
    let _lock = env_lock();
    let path = unique_path("pixie-config", "toml");
    fs::write(&path, "addr = \"0.0.0.0:9091\"\nworkers = 5\n")
        .expect("could not write TOML config");
    let _config = EnvGuard::set("PIXIE_CONFIG", path.to_str().expect("path is not UTF-8"));

    let config = runtime_config().expect("invalid TOML runtime configuration");
    assert_eq!(config.addr, "0.0.0.0:9091");
    assert_eq!(config.workers, 5);

    fs::remove_file(path).expect("could not remove TOML config");
}

/// Verifies that a JSON file is selected from its extension.
#[test]
fn runtime_config_reads_json_file() {
    let _lock = env_lock();
    let path = unique_path("pixie-config", "json");
    fs::write(&path, r#"{"addr":"0.0.0.0:9092","workers":6}"#)
        .expect("could not write JSON config");
    let _config = EnvGuard::set("PIXIE_CONFIG", path.to_str().expect("path is not UTF-8"));

    let config = runtime_config().expect("invalid JSON runtime configuration");
    assert_eq!(config.addr, "0.0.0.0:9092");
    assert_eq!(config.workers, 6);

    fs::remove_file(path).expect("could not remove JSON config");
}

/// Verifies that unsupported config extensions produce a clear error.
#[test]
fn runtime_config_rejects_unknown_file_extension() {
    let _lock = env_lock();
    let path = unique_path("pixie-config", "conf");
    fs::write(&path, "addr = 127.0.0.1:9090").expect("could not write config");
    let _config = EnvGuard::set("PIXIE_CONFIG", path.to_str().expect("path is not UTF-8"));

    let err = runtime_config().expect_err("expected unsupported extension error");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
    assert!(err.to_string().contains("use .yml, .yaml, .toml, or .json"));

    fs::remove_file(path).expect("could not remove config");
}

/// Verifies the worker fallback when `workers` is zero.
#[test]
fn runtime_config_ignore_workers_zero() {
    let _lock = env_lock();
    let config_file = TempFile::new("pixie-config", "addr: 127.0.0.1:8080\nworkers: 0\n");
    let _config = EnvGuard::set("PIXIE_CONFIG", config_file.as_str());

    let config = runtime_config().expect("configuration runtime invalide");
    assert_eq!(config.addr, "127.0.0.1:8080");
    assert_eq!(config.workers, 4);
}

/// Verifies the YAML aliases `address` and `nb_worker`.
#[test]
fn runtime_config_supporte_les_alias_yaml() {
    let _lock = env_lock();
    let config_file = TempFile::new("pixie-config", "address: 10.0.0.1:8081\nnb_worker: 6\n");
    let _config = EnvGuard::set("PIXIE_CONFIG", config_file.as_str());

    let config = runtime_config().expect("configuration runtime invalide");
    assert_eq!(config.addr, "10.0.0.1:8081");
    assert_eq!(config.workers, 6);
}

/// Verifies that invalid YAML returns an error.
#[test]
fn runtime_config_remonte_une_erreur_yaml() {
    let _lock = env_lock();
    let config_file = TempFile::new("pixie-config", ":\n  - not-valid");
    let _config = EnvGuard::set("PIXIE_CONFIG", config_file.as_str());

    let err = runtime_config().expect_err("expected a YAML error");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert!(err.to_string().contains("invalid YAML"));
}

/// Verifies that an empty `PIXIE_CONFIG` value is ignored.
#[test]
fn runtime_config_ignore_pixie_config_vide() {
    let _lock = env_lock();
    let _config = EnvGuard::set("PIXIE_CONFIG", "");

    let config = runtime_config().expect("configuration runtime invalide");
    assert!(!config.addr.is_empty());
    assert!(config.workers > 0);
}

/// Verifies that a local `config-pixie.yml` file is supported.
#[test]
fn runtime_config_reads_local_file() {
    let _lock = env_lock();
    let _config = EnvGuard::set("PIXIE_CONFIG", "");

    let path = std::env::current_dir()
        .expect("current directory not found")
        .join("config-pixie.yml");

    fs::write(&path, "addr: 127.0.0.1:7070\nworkers: 3\n")
        .expect("could not write local config-pixie.yml");

    let config = runtime_config().expect("configuration runtime invalide");
    assert_eq!(config.addr, "127.0.0.1:7070");
    assert_eq!(config.workers, 3);

    let _ = fs::remove_file(path);
}
