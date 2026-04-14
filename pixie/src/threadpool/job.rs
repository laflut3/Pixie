/// Type d'un travail asynchrone exécuté par un worker.
pub(crate) type Job = Box<dyn FnOnce() + Send + 'static>;
