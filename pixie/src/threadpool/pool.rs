use std::sync::{Arc, Mutex, mpsc};

use super::{job::Job, worker::Worker};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
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

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        match &self.sender {
            Some(sender) => {
                if let Err(err) = sender.send(job) {
                    eprintln!("[pixie][error] failed to send job to worker: {err}");
                }
            }
            None => {
                eprintln!("[pixie][warn] thread pool is shut down; rejecting new job");
            }
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            worker.join();
        }
    }
}
