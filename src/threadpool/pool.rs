use std::sync::{Arc, Mutex, mpsc};

use super::{Job, worker::Worker};

/// Fixed-size thread pool for concurrent jobs.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Creates a thread pool of size `size`.
    ///
    /// Panics when `size == 0`.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "thread pool size must be greater than zero");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Submits a job to the pool for asynchronous execution.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let Some(sender) = self.sender.as_ref() else {
            crate::logger::log_warn(format_args!("thread pool is shutting down; dropping job"));
            return;
        };

        if let Err(err) = sender.send(Box::new(f)) {
            crate::logger::log_error(format_args!("failed to send job to worker: {err}"));
        }
    }
}

impl Drop for ThreadPool {
    /// Closes the channel cleanly, then waits for all workers to finish.
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            worker.join();
        }
    }
}
