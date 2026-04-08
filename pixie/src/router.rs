use std::{
    env,
    path::{Path, PathBuf},
};

use super::{log_warn};

const DEFAULT_WEB_ROOT: &str = "/usr/share/pixie/web";
const INDEX_PAGE: &str = "hello.html";
const NOT_FOUND_PAGE: &str = "404.html";

pub fn resolve_web_root() -> PathBuf {
    if let Ok(path) = env::var("PIXIE_WEB_ROOT") {
        let trimmed = path.trim();
        
        if !trimmed.is_empty() {
            let candidate = PathBuf::from(trimmed);
            if candidate.is_dir() {
                return candidate;
            }
            log_warn(format_args!(
                "PIXIE_WEB_ROOT='{}' is not a directory, using fallback",
                candidate.display()
            ));
        }
    }

    let dev_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../web");
    if dev_root.is_dir() {
        return dev_root;
    }

    PathBuf::from(DEFAULT_WEB_ROOT)
}

pub fn resolve_route(request_line: &str, web_root: &Path) -> (&'static str, PathBuf) {
    let not_found = || ("HTTP/1.1 404 NOT FOUND", web_root.join(NOT_FOUND_PAGE));

    if !request_line.starts_with("GET /") {
        return not_found();
    }

    let route = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .trim_start_matches('/')
        .split('?')
        .next()
        .unwrap_or("");

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
