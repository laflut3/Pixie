use std::sync::{Arc, Mutex, mpsc};

use super::{Job, worker::Worker};

/// Pool de threads fixe utilisé pour exécuter des jobs concurrents.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Crée un thread-pool de taille `size`.
    ///
    /// Panique si `size == 0`.
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

    /// Soumet un job au pool pour exécution asynchrone.
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
    /// Ferme proprement le canal puis attend la terminaison de tous les workers.
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            worker.join();
        }
    }
}
