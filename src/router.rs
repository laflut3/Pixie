//! Safe file routing shared by every HTTP protocol.
use crate::{mime::content_type, sites::Sites};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn resolve_web_root() -> PathBuf {
    Sites::from_env().resolve(None)
}

/// Compatibility entry point for callers routing an HTTP request line.
pub fn resolve_route(request_line: &str, web_root: &Path) -> (&'static str, PathBuf) {
    let mut parts = request_line.split_whitespace();
    if !matches!(parts.next(), Some("GET" | "HEAD")) {
        return not_found(web_root);
    }
    resolve_path(parts.next().unwrap_or(""), web_root, false)
}

fn not_found(root: &Path) -> (&'static str, PathBuf) {
    // A symlinked error page is never served.
    (
        "HTTP/1.1 404 NOT FOUND",
        safe_file(root, Path::new("404.html")).unwrap_or_else(|| root.join(".pixie-missing")),
    )
}

pub fn resolve_path(target: &str, root: &Path, spa: bool) -> (&'static str, PathBuf) {
    let Some(path) = decode_path(target.split('?').next().unwrap_or("")) else {
        return not_found(root);
    };
    let relative = Path::new(path.trim_start_matches('/'));
    let mut candidates = Vec::new();
    if path.ends_with('/') || relative.as_os_str().is_empty() {
        candidates.push(relative.join("index.html"));
        if relative.as_os_str().is_empty() {
            candidates.push(PathBuf::from("hello.html"));
        }
    } else {
        candidates.push(relative.to_path_buf());
        candidates.push(relative.join("index.html"));
        if relative.extension().is_none() {
            candidates.push(relative.with_extension("html"));
        }
    }
    for candidate in candidates {
        if let Some(file) = safe_file(root, &candidate) {
            return ("HTTP/1.1 200 OK", file);
        }
    }
    if spa && relative.extension().is_none() {
        if let Some(file) = safe_file(root, Path::new("index.html")) {
            return ("HTTP/1.1 200 OK", file);
        }
    }
    not_found(root)
}

/// Decode once, reject dot/hidden components and separators introduced by encoding.
fn decode_path(target: &str) -> Option<String> {
    if !target.starts_with('/') || target.starts_with("//") {
        return None;
    }
    let mut decoded = Vec::new();
    let raw = target.as_bytes();
    let mut i = 0;
    while i < raw.len() {
        let byte = if raw[i] == b'%' {
            let hex = std::str::from_utf8(raw.get(i + 1..i + 3)?).ok()?;
            let byte = u8::from_str_radix(hex, 16).ok()?;
            if matches!(byte, b'/' | b'\\' | b'%') {
                return None;
            }
            i += 2;
            byte
        } else {
            raw[i]
        };
        if byte < 32 || byte == 127 || byte == b'\\' || byte == b':' {
            return None;
        }
        decoded.push(byte);
        i += 1;
    }
    let path = String::from_utf8(decoded).ok()?;
    if path.split('/').any(|part| part.starts_with('.')) {
        return None;
    }
    Some(path)
}

fn safe_file(root: &Path, relative: &Path) -> Option<PathBuf> {
    let mut candidate = root.to_path_buf();
    if root.is_symlink() {
        return None;
    }
    for component in relative.components() {
        let std::path::Component::Normal(part) = component else {
            return None;
        };
        candidate.push(part);
        if fs::symlink_metadata(&candidate)
            .ok()?
            .file_type()
            .is_symlink()
        {
            return None;
        }
    }
    if candidate.is_file() {
        Some(candidate)
    } else {
        None
    }
}

pub struct StaticResponse {
    pub status: u16,
    pub body: Vec<u8>,
    pub content_type: &'static str,
}

pub fn read_response(
    method: &str,
    path: &str,
    host: Option<&str>,
    sites: &Sites,
) -> StaticResponse {
    if method != "GET" && method != "HEAD" {
        return StaticResponse {
            status: 405,
            body: b"Method not allowed".to_vec(),
            content_type: "text/plain; charset=utf-8",
        };
    }
    let root = sites.resolve(host);
    let (status, file) = resolve_path(path, &root, sites.spa_fallback);
    if file.file_name() == Some(std::ffi::OsStr::new(".pixie-missing")) {
        return StaticResponse {
            status: 404,
            body: b"Not Found".to_vec(),
            content_type: "text/plain; charset=utf-8",
        };
    }
    // Capability-relative reads prevent symlink replacement from escaping the site.
    let bytes = (|| {
        let directory = cap_std::fs::Dir::open_ambient_dir(&root, cap_std::ambient_authority())?;
        let relative = file.strip_prefix(&root).map_err(std::io::Error::other)?;
        directory.read(relative)
    })();
    match bytes {
        Ok(body) => StaticResponse {
            status: if status.contains("200") { 200 } else { 404 },
            content_type: content_type(&file),
            body,
        },
        Err(error) => {
            let status = if status.contains("404") || error.kind() == std::io::ErrorKind::NotFound {
                404
            } else {
                500
            };
            StaticResponse {
                status,
                body: if status == 404 {
                    b"Not Found".to_vec()
                } else {
                    b"Internal Server Error".to_vec()
                },
                content_type: "text/plain; charset=utf-8",
            }
        }
    }
}
