mod common;

use common::{EnvGuard, TempFile, env_lock, unique_path};
use std::fs;
use pixie::runtime_config;

/// Vérifie le fallback complet quand aucun fichier n'est disponible.
#[test]
fn runtime_config_retourne_les_valeurs_par_defaut() {
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

/// Vérifie la lecture d'un fichier YAML via `PIXIE_CONFIG`.
#[test]
fn runtime_config_lit_le_fichier_pointe_par_pixie_config() {
    let _lock = env_lock();
    let config_file = TempFile::new("pixie-config", "host: 0.0.0.0\nport: 9090\nworkers: 8\n");
    let _config = EnvGuard::set("PIXIE_CONFIG", config_file.as_str());

    let config = runtime_config().expect("configuration runtime invalide");
    assert_eq!(config.addr, "0.0.0.0:9090");
    assert_eq!(config.workers, 8);
}

/// Vérifie le fallback worker quand `workers` vaut 0.
#[test]
fn runtime_config_ignore_workers_zero() {
    let _lock = env_lock();
    let config_file = TempFile::new("pixie-config", "addr: 127.0.0.1:8080\nworkers: 0\n");
    let _config = EnvGuard::set("PIXIE_CONFIG", config_file.as_str());

    let config = runtime_config().expect("configuration runtime invalide");
    assert_eq!(config.addr, "127.0.0.1:8080");
    assert_eq!(config.workers, 4);
}

/// Vérifie les alias YAML `address` et `nb_worker`.
#[test]
fn runtime_config_supporte_les_alias_yaml() {
    let _lock = env_lock();
    let config_file = TempFile::new("pixie-config", "address: 10.0.0.1:8081\nnb_worker: 6\n");
    let _config = EnvGuard::set("PIXIE_CONFIG", config_file.as_str());

    let config = runtime_config().expect("configuration runtime invalide");
    assert_eq!(config.addr, "10.0.0.1:8081");
    assert_eq!(config.workers, 6);
}

/// Vérifie la remontée d'erreur quand le YAML est invalide.
#[test]
fn runtime_config_remonte_une_erreur_yaml() {
    let _lock = env_lock();
    let config_file = TempFile::new("pixie-config", ":\n  - not-valid");
    let _config = EnvGuard::set("PIXIE_CONFIG", config_file.as_str());

    let err = runtime_config().expect_err("une erreur YAML était attendue");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    assert!(err.to_string().contains("invalid YAML"));
}

/// Vérifie qu'une valeur vide de `PIXIE_CONFIG` est ignorée.
#[test]
fn runtime_config_ignore_pixie_config_vide() {
    let _lock = env_lock();
    let _config = EnvGuard::set("PIXIE_CONFIG", "");

    let config = runtime_config().expect("configuration runtime invalide");
    assert!(!config.addr.is_empty());
    assert!(config.workers > 0);
}

/// Vérifie que le fichier local `config-pixie.yml` est pris en charge.
#[test]
fn runtime_config_lit_le_fichier_local() {
    let _lock = env_lock();
    let _config = EnvGuard::set("PIXIE_CONFIG", "");

    let path = std::env::current_dir()
        .expect("répertoire courant introuvable")
        .join("config-pixie.yml");

    fs::write(&path, "addr: 127.0.0.1:7070\nworkers: 3\n")
        .expect("écriture du config-pixie.yml local impossible");

    let config = runtime_config().expect("configuration runtime invalide");
    assert_eq!(config.addr, "127.0.0.1:7070");
    assert_eq!(config.workers, 3);

    let _ = fs::remove_file(path);
}
