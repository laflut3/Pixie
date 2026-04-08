use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

use super::job::Job;

pub(super) struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub(super) fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let message = match receiver.lock() {
                    Ok(guard) => guard.recv(),
                    Err(_) => {
                        eprintln!("[pixie][error] worker {id} receiver lock poisoned; shutting down");
                        break;
                    }
                };

                match message {
                    Ok(job) => {
                        job();
                    }
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

    pub(super) fn join(&mut self) {
        if let Some(thread) = self.thread.take() {
            if let Err(err) = thread.join() {
                eprintln!("[pixie][error] worker {} panicked: {:?}", self.id, err);
            }
        }
    }
}
