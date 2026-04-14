//! Thread-pool minimal utilisé par le serveur pour traiter les connexions en parallèle.

mod job;
mod pool;
mod worker;

pub use pool::ThreadPool;
