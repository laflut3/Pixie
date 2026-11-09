mod common;

use std::path::Path;

use common::{EnvGuard, TempDir, env_lock};
use pixie::{resolve_route, resolve_web_root};

/// Verifies web root resolution through `PIXIE_WEB_ROOT`.
#[test]
fn resolve_web_root_utilise_pixie_web_root_si_valide() {
    let _lock = env_lock();
    let web = TempDir::new("pixie-web-root");
    let _env = EnvGuard::set(
        "PIXIE_WEB_ROOT",
        web.path.to_str().expect("chemin non UTF-8"),
    );

    assert_eq!(resolve_web_root(), web.path);
}

/// Verifies that an empty variable is ignored and the development fallback is used.
#[test]
fn resolve_web_root_ignore_pixie_web_root_vide() {
    let _lock = env_lock();
    let _env = EnvGuard::set("PIXIE_WEB_ROOT", "");

    let expected = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
    assert_eq!(resolve_web_root(), expected);
}

/// Verifies that an invalid `PIXIE_WEB_ROOT` value is ignored.
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

    let expected = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
    assert_eq!(resolve_web_root(), expected);
}

/// Verifies the system fallback when the development `web` directory is unavailable.
#[test]
fn resolve_web_root_fallback_systeme_si_dossier_dev_absent() {
    let _lock = env_lock();
    let _env = EnvGuard::set("PIXIE_WEB_ROOT", "");

    let dev_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
    let backup = Path::new(env!("CARGO_MANIFEST_DIR")).join("web-test-backup");

    if backup.exists() {
        let _ = std::fs::remove_dir_all(&backup);
    }

    std::fs::rename(&dev_root, &backup).expect("renommage temporaire du dossier web impossible");

    let resolved = resolve_web_root();

    std::fs::rename(&backup, &dev_root).expect("restauration du dossier web impossible");

    assert_eq!(resolved, Path::new("/usr/share/pixie/web"));
}

/// Verifies that `/` serves `hello.html` when the file exists.
#[test]
fn resolve_route_sert_la_page_index() {
    let web = TempDir::new("pixie-web-index");
    web.write("hello.html", "ok");
    web.write("404.html", "ko");

    let (status, file) = resolve_route("GET / HTTP/1.1", &web.path);
    assert_eq!(status, "HTTP/1.1 200 OK");
    assert_eq!(file, web.path.join("hello.html"));
}

/// Verifies that `/about` serves `about.html`.
#[test]
fn resolve_route_sert_une_page_nommee() {
    let web = TempDir::new("pixie-web-about");
    web.write("about.html", "about");
    web.write("404.html", "ko");

    let (status, file) = resolve_route("GET /about HTTP/1.1", &web.path);
    assert_eq!(status, "HTTP/1.1 200 OK");
    assert_eq!(file, web.path.join("about.html"));
}

/// Verifies that the query string is ignored when resolving a file.
#[test]
fn resolve_route_ignore_la_query_string() {
    let web = TempDir::new("pixie-web-query");
    web.write("hello.html", "ok");
    web.write("404.html", "ko");

    let (status, file) = resolve_route("GET /?name=pixie HTTP/1.1", &web.path);
    assert_eq!(status, "HTTP/1.1 200 OK");
    assert_eq!(file, web.path.join("hello.html"));
}

/// Verifies the 404 fallback when the requested route does not exist.
#[test]
fn resolve_route_returns_404_for_missing_file() {
    let web = TempDir::new("pixie-web-missing");
    web.write("404.html", "ko");

    let (status, file) = resolve_route("GET /missing HTTP/1.1", &web.path);
    assert_eq!(status, "HTTP/1.1 404 NOT FOUND");
    assert_eq!(file, web.path.join("404.html"));
}

/// Verifies that a non-GET method returns 404.
#[test]
fn resolve_route_refuse_les_methodes_non_get() {
    let web = TempDir::new("pixie-web-method");
    web.write("404.html", "ko");

    let (status, file) = resolve_route("POST / HTTP/1.1", &web.path);
    assert_eq!(status, "HTTP/1.1 404 NOT FOUND");
    assert_eq!(file, web.path.join("404.html"));
}
