use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

use super::job::Job;

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

                match message {
                    Ok(job) => job(),
                    Err(_) => {
                        eprintln!("[pixie][info] worker {id} disconnected; shutting down");
                        break;
                    }
                }
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
            if let Err(err) = thread.join() {
                eprintln!("[pixie][error] worker {} panicked: {:?}", self.id, err);
            }
        }
    }
}
