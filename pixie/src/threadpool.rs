//! Thread-pool minimal utilisé par le serveur pour traiter les connexions en parallèle.

mod pool;
mod worker;

/// Type d'un travail asynchrone exécuté par un worker.
pub(crate) type Job = Box<dyn FnOnce() + Send + 'static>;

pub use pool::ThreadPool;
