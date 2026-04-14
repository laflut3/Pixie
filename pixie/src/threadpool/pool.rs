use std::sync::{Arc, Mutex, mpsc};

use super::{job::Job, worker::Worker};

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
        let sender = self
            .sender
            .as_ref()
            .expect("thread pool sender is missing");

        if let Err(err) = sender.send(Box::new(f)) {
            eprintln!("[pixie][error] failed to send job to worker: {err}");
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
