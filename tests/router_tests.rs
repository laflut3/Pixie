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

#[test]
fn static_assets_nested_indexes_and_spa_routes() {
    use pixie::router::resolve_path;
    let web = TempDir::new("pixie-assets");
    web.write("index.html", "app");
    std::fs::create_dir(web.path.join("assets")).unwrap();
    web.write("assets/app.js", "console.log('app')");
    assert_eq!(
        resolve_path("/", &web.path, false).1,
        web.path.join("index.html")
    );
    assert_eq!(
        resolve_path("/assets/app.js?v=1", &web.path, false).1,
        web.path.join("assets/app.js")
    );
    assert_eq!(
        resolve_path("/dashboard", &web.path, true).1,
        web.path.join("index.html")
    );
    assert!(
        resolve_path("/missing.js", &web.path, true)
            .0
            .contains("404")
    );
    assert!(
        resolve_path("/dashboard", &web.path, false)
            .0
            .contains("404")
    );
}

#[test]
fn traversal_hidden_and_encoded_separator_paths_are_rejected() {
    use pixie::router::resolve_path;
    let web = TempDir::new("pixie-path-safety");
    web.write("index.html", "app");
    for path in [
        "/../index.html",
        "/%2e%2e/index.html",
        "/.env",
        "/a%2fb",
        "/a%5cb",
        "/a\\b",
        "/%00",
        "/%252e%252e",
        "//index.html",
    ] {
        assert!(
            resolve_path(path, &web.path, true).0.contains("404"),
            "{path}"
        );
    }
}

#[test]
fn sites_are_discovered_dynamically_and_assets_keep_binary_bytes() {
    use pixie::{router::read_response, sites::Sites};
    let _lock = env_lock();
    let sites = TempDir::new("pixie-sites");
    let _web = EnvGuard::set("PIXIE_WEB_ROOT", "");
    let _sites = EnvGuard::set("PIXIE_SITES_ROOT", sites.path.to_str().unwrap());
    let _spa = EnvGuard::set("PIXIE_SPA_FALLBACK", "true");
    std::fs::create_dir(sites.path.join("default")).unwrap();
    sites.write("default/index.html", "default");
    let config = Sites::from_env();
    assert_eq!(
        read_response("GET", "/", Some("unknown.test"), &config).body,
        b"default"
    );
    // No restart or directory cache: add a host after the settings were loaded.
    std::fs::create_dir(sites.path.join("example.com")).unwrap();
    sites.write("example.com/index.html", "named");
    std::fs::write(sites.path.join("example.com/icon.png"), [0, 255, 128]).unwrap();
    assert_eq!(
        read_response("GET", "/", Some("EXAMPLE.com:8080"), &config).body,
        b"named"
    );
    let image = read_response("GET", "/icon.png", Some("example.com"), &config);
    assert_eq!(image.content_type, "image/png");
    assert_eq!(image.body, [0, 255, 128]);
    assert_eq!(
        read_response("GET", "/dashboard", Some("example.com"), &config).body,
        b"named"
    );
    assert_eq!(
        read_response("GET", "/missing.js", Some("example.com"), &config).status,
        404
    );
    assert_eq!(
        read_response("GET", "/", Some("../../secret"), &config).body,
        b"default"
    );
}

#[cfg(unix)]
#[test]
fn symlinks_cannot_expose_files_outside_the_site() {
    use pixie::{
        router::{read_response, resolve_path},
        sites::Sites,
    };
    let _lock = env_lock();
    let web = TempDir::new("pixie-symlink-web");
    let outside = TempDir::new("pixie-outside");
    outside.write("secret.txt", "secret");
    std::os::unix::fs::symlink(&outside.path, web.path.join("linked")).unwrap();
    std::os::unix::fs::symlink(outside.path.join("secret.txt"), web.path.join("404.html")).unwrap();
    let _web = EnvGuard::set("PIXIE_WEB_ROOT", web.path.to_str().unwrap());
    let config = Sites::from_env();
    assert!(
        resolve_path("/linked/secret.txt", &web.path, false)
            .0
            .contains("404")
    );
    let result = read_response("GET", "/linked/secret.txt", None, &config);
    assert_eq!(result.status, 404);
    assert_eq!(result.body, b"Not Found");
}

#[test]
fn mime_types_cover_react_assets_and_unknown_binary_files() {
    use pixie::mime::content_type;
    for (name, mime) in [
        ("app.js", "text/javascript; charset=utf-8"),
        ("app.css", "text/css; charset=utf-8"),
        ("logo.svg", "image/svg+xml"),
        ("font.woff2", "font/woff2"),
        ("app.wasm", "application/wasm"),
        ("file.unknown", "application/octet-stream"),
    ] {
        assert_eq!(content_type(Path::new(name)), mime);
    }
}
