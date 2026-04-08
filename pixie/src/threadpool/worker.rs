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
                        eprintln!("Worker {id} receiver lock poisoned; shutting down.");
                        break;
                    }
                };

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
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
                eprintln!("Worker {} panicked: {:?}", self.id, err);
            }
        }
    }
}
