use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

use super::Job;

/// Worker interne du thread-pool.
pub(super) struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// Démarre un worker qui consomme des jobs depuis `receiver`.
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

    /// Attend la fin du thread worker.
    pub(super) fn join(&mut self) {
        if let Some(thread) = self.thread.take() {
            if thread.join().is_err() {
                crate::logger::log_error(format_args!("worker {} panicked", self.id));
            }
        }
    }
}
