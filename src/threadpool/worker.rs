use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

use super::Job;

/// Internal worker in the thread pool.
pub(super) struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Starts a worker that consumes jobs from `receiver`.
    pub(super) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = receiver
                    .lock()
                    .expect("worker receiver lock poisoned")
                    .recv();
                let Ok(job) = message else {
                    crate::logger::log_info(format_args!(
                        "worker {id} disconnected; shutting down"
                    ));
                    break;
                };

                job();
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }

    /// Waits for the worker thread to finish.
    pub(super) fn join(&mut self) {
        if let Some(thread) = self.thread.take() {
            if thread.join().is_err() {
                crate::logger::log_error(format_args!("worker {} panicked", self.id));
            }
        }
    }
}
