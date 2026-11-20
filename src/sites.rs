//! Site directory selection. Directories are discovered on each request.
use std::{
    env,
    path::{Path, PathBuf},
};

use crate::logger::log_warn;

pub const DEFAULT_SITES_ROOT: &str = "/etc/pixie/sites";

#[derive(Clone, Debug)]
pub struct Sites {
    root: PathBuf,
    single: bool,
    pub spa_fallback: bool,
}

impl Sites {
    pub fn from_env() -> Self {
        let spa_fallback = env::var("PIXIE_SPA_FALLBACK").is_ok_and(|value| value == "true");
        if let Some(root) = env::var_os("PIXIE_WEB_ROOT").filter(|value| !value.is_empty()) {
            let root = PathBuf::from(root);
            if root.is_dir() {
                return Self {
                    root,
                    single: true,
                    spa_fallback,
                };
            }
            log_warn(format_args!(
                "PIXIE_WEB_ROOT is not a directory, using site discovery"
            ));
        }
        let configured_root = env::var_os("PIXIE_SITES_ROOT").filter(|value| !value.is_empty());
        let root = configured_root
            .as_ref()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_SITES_ROOT));
        if root.is_dir() || configured_root.is_some() {
            Self {
                root,
                single: false,
                spa_fallback,
            }
        } else {
            let dev = Path::new(env!("CARGO_MANIFEST_DIR")).join("web");
            Self {
                root: if dev.is_dir() {
                    dev
                } else {
                    PathBuf::from("/usr/share/pixie/web")
                },
                single: true,
                spa_fallback,
            }
        }
    }

    pub fn directory(&self) -> &Path {
        &self.root
    }

    pub fn resolve(&self, host: Option<&str>) -> PathBuf {
        if self.single {
            return self.root.clone();
        }
        if let Some(host) = host.and_then(hostname) {
            let candidate = self.root.join(host);
            if candidate.is_dir() && !candidate.is_symlink() {
                return candidate;
            }
        }
        self.root.join("default")
    }
}

/// Only DNS-style host names can become directory names. Ports and case are normalized.
fn hostname(host: &str) -> Option<String> {
    let authority = host.parse::<hyper::http::uri::Authority>().ok()?;
    let name = authority.host().trim_end_matches('.').to_ascii_lowercase();
    if name.is_empty()
        || name.len() > 253
        || name.split('.').any(|label| {
            label.is_empty()
                || label.len() > 63
                || label.starts_with('-')
                || label.ends_with('-')
                || !label
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-')
        })
    {
        return None;
    }
    Some(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn host_names_are_safe_directory_names() {
        assert_eq!(hostname("EXAMPLE.com:8080"), Some("example.com".into()));
        for bad in ["../secret", "a/b", "a\\b", "a%2fb", "-bad.com", "[::1]"] {
            assert_eq!(hostname(bad), None, "{bad}");
        }
    }
}
