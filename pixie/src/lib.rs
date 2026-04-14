//! Bibliothèque principale de Pixie.
//!
//! Cette crate expose un serveur HTTP statique minimal:
//! - résolution de configuration runtime
//! - résolution de routes HTML
//! - exécution serveur TCP avec thread-pool

pub mod config;
pub mod logger;
pub mod router;
pub mod server;
pub mod threadpool;

pub use crate::config::RuntimeConfig;
pub use crate::config::runtime_config;
pub use crate::logger::log_error;
pub use crate::logger::log_info;
pub use crate::logger::log_warn;
pub use crate::router::resolve_route;
pub use crate::router::resolve_web_root;
pub use crate::server::run_server;
pub use crate::threadpool::ThreadPool;
