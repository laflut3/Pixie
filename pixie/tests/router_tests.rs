mod common;

use std::path::Path;

use common::{EnvGuard, TempDir, env_lock};
use pixie::{resolve_route, resolve_web_root};

/// Vérifie la résolution de la racine web via `PIXIE_WEB_ROOT`.
#[test]
fn resolve_web_root_utilise_pixie_web_root_si_valide() {
    let _lock = env_lock();
    let web = TempDir::new("pixie-web-root");
    let _env = EnvGuard::set("PIXIE_WEB_ROOT", web.path.to_str().expect("chemin non UTF-8"));

    assert_eq!(resolve_web_root(), web.path);
}

/// Vérifie que la variable vide est ignorée et que le fallback dev est pris.
#[test]
fn resolve_web_root_ignore_pixie_web_root_vide() {
    let _lock = env_lock();
    let _env = EnvGuard::set("PIXIE_WEB_ROOT", "");

    let expected = Path::new(env!("CARGO_MANIFEST_DIR")).join("../web");
    assert_eq!(resolve_web_root(), expected);
}

/// Vérifie qu'une valeur invalide de `PIXIE_WEB_ROOT` est ignorée.
#[test]
fn resolve_web_root_ignore_pixie_web_root_non_dossier() {
    let _lock = env_lock();
    let invalid_file = TempDir::new("pixie-web-root-invalid");
    invalid_file.write("file.txt", "not a dir");

    let invalid_path = invalid_file.path.join("file.txt");
    let _env = EnvGuard::set(
        "PIXIE_WEB_ROOT",
        invalid_path.to_str().expect("chemin non UTF-8"),
    );

    let expected = Path::new(env!("CARGO_MANIFEST_DIR")).join("../web");
    assert_eq!(resolve_web_root(), expected);
}

/// Vérifie le fallback système quand le dossier dev `../web` est indisponible.
#[test]
fn resolve_web_root_fallback_systeme_si_dossier_dev_absent() {
    let _lock = env_lock();
    let _env = EnvGuard::set("PIXIE_WEB_ROOT", "");

    let dev_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../web");
    let backup = Path::new(env!("CARGO_MANIFEST_DIR")).join("../web-test-backup");

    if backup.exists() {
        let _ = std::fs::remove_dir_all(&backup);
    }

    std::fs::rename(&dev_root, &backup).expect("renommage temporaire du dossier web impossible");

    let resolved = resolve_web_root();

    std::fs::rename(&backup, &dev_root).expect("restauration du dossier web impossible");

    assert_eq!(resolved, Path::new("/usr/share/pixie/web"));
}

/// Vérifie que `/` sert `hello.html` quand le fichier existe.
#[test]
fn resolve_route_sert_la_page_index() {
    let web = TempDir::new("pixie-web-index");
    web.write("hello.html", "ok");
    web.write("404.html", "ko");

    let (status, file) = resolve_route("GET / HTTP/1.1", &web.path);
    assert_eq!(status, "HTTP/1.1 200 OK");
    assert_eq!(file, web.path.join("hello.html"));
}

/// Vérifie que `/about` sert `about.html`.
#[test]
fn resolve_route_sert_une_page_nommee() {
    let web = TempDir::new("pixie-web-about");
    web.write("about.html", "about");
    web.write("404.html", "ko");

    let (status, file) = resolve_route("GET /about HTTP/1.1", &web.path);
    assert_eq!(status, "HTTP/1.1 200 OK");
    assert_eq!(file, web.path.join("about.html"));
}

/// Vérifie que la query string est ignorée dans la résolution du fichier.
#[test]
fn resolve_route_ignore_la_query_string() {
    let web = TempDir::new("pixie-web-query");
    web.write("hello.html", "ok");
    web.write("404.html", "ko");

    let (status, file) = resolve_route("GET /?name=pixie HTTP/1.1", &web.path);
    assert_eq!(status, "HTTP/1.1 200 OK");
    assert_eq!(file, web.path.join("hello.html"));
}

/// Vérifie le fallback 404 quand la route demandée n'existe pas.
#[test]
fn resolve_route_retourne_404_si_fichier_absent() {
    let web = TempDir::new("pixie-web-missing");
    web.write("404.html", "ko");

    let (status, file) = resolve_route("GET /missing HTTP/1.1", &web.path);
    assert_eq!(status, "HTTP/1.1 404 NOT FOUND");
    assert_eq!(file, web.path.join("404.html"));
}

/// Vérifie qu'une méthode non GET retourne 404.
#[test]
fn resolve_route_refuse_les_methodes_non_get() {
    let web = TempDir::new("pixie-web-method");
    web.write("404.html", "ko");

    let (status, file) = resolve_route("POST / HTTP/1.1", &web.path);
    assert_eq!(status, "HTTP/1.1 404 NOT FOUND");
    assert_eq!(file, web.path.join("404.html"));
}
