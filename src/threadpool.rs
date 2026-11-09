//! Minimal worker pool used by the server to process connections concurrently.

mod pool;
mod worker;

/// Type of asynchronous job executed by a worker.
pub(crate) type Job = Box<dyn FnOnce() + Send + 'static>;

pub use pool::ThreadPool;
