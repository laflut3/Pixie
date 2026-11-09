//! HTTP routing to static HTML files.
//!
//! This module:
//! - resolves the web root (`PIXIE_WEB_ROOT` -> development -> system)
//! - maps an HTTP GET request line to an `.html` file
//! - returns `404.html` when a route is invalid or missing

use std::{
    env,
    path::{Path, PathBuf},
};

use crate::logger::log_warn;

const DEFAULT_WEB_ROOT: &str = "/usr/share/pixie/web";
const INDEX_PAGE: &str = "hello.html";
const NOT_FOUND_PAGE: &str = "404.html";

/// Resolves the web root used by the server.
///
/// Priority:
/// 1. `PIXIE_WEB_ROOT` when it points to an existing directory
/// 2. `web` relative to the Cargo manifest (development mode)
/// 3. `/usr/share/pixie/web` (system installation)
pub fn resolve_web_root() -> PathBuf {
    if let Some(path) = env::var("PIXIE_WEB_ROOT")
        .ok()
        .filter(|path| !path.is_empty())
    {
        let candidate = PathBuf::from(path);

        if candidate.is_dir() {
            return candidate;
        }

        log_warn(format_args!(
            "PIXIE_WEB_ROOT='{}' is not a directory, using fallback",
            candidate.display()
        ));
    }

    let dev_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");

    if dev_root.is_dir() {
        return dev_root;
    }

    PathBuf::from(DEFAULT_WEB_ROOT)
}

/// Resolves an HTTP request line to a status and file path.
///
/// Examples:
/// - `GET / HTTP/1.1` -> `hello.html`
/// - `GET /about HTTP/1.1` -> `about.html`
/// - invalid or missing route -> `404.html`
pub fn resolve_route(request_line: &str, web_root: &Path) -> (&'static str, PathBuf) {
    let not_found = || ("HTTP/1.1 404 NOT FOUND", web_root.join(NOT_FOUND_PAGE));

    let Some(route) = extract_route(request_line) else {
        return not_found();
    };

    let file = if route.is_empty() {
        web_root.join(INDEX_PAGE)
    } else {
        web_root.join(format!("{route}.html"))
    };

    if file.is_file() {
        ("HTTP/1.1 200 OK", file)
    } else {
        not_found()
    }
}

/// Extracts the route from an HTTP GET request line.
///
/// Returns:
/// - `Some("")` for `/`
/// - `Some("about")` for `/about` or `/about?x=1`
/// - `None` if the method is not GET or the request line is invalid
fn extract_route(request_line: &str) -> Option<&str> {
    let mut parts = request_line.split_whitespace();
    let method = parts.next()?;
    let target = parts.next()?;

    if method != "GET" || !target.starts_with('/') {
        return None;
    }

    let path = target.split_once('?').map_or(target, |(path, _)| path);
    Some(path.trim_start_matches('/'))
}
