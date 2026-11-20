//! Pixie core library.
//!
//! This crate exposes a minimal static HTTP server:
//! - runtime configuration resolution
//! - HTML route resolution
//! - TCP server execution with a worker pool

pub mod config;
pub mod logger;
pub mod router;
pub mod server;
pub mod threadpool;
pub mod tls;

pub use crate::config::runtime_config;
pub use crate::config::{RuntimeConfig, TlsConfig};
pub use crate::logger::log_error;
pub use crate::logger::log_info;
pub use crate::logger::log_warn;
pub use crate::router::resolve_route;
pub use crate::router::resolve_web_root;
pub use crate::server::run_server;
pub use crate::threadpool::ThreadPool;
