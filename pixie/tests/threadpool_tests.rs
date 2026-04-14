use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
    time::Duration,
};

use pixie::ThreadPool;

/// Vérifie que le pool exécute tous les jobs soumis.
#[test]
fn thread_pool_execute_tous_les_jobs() {
    let pool = ThreadPool::new(2);
    let done = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = mpsc::channel();

    for _ in 0..8 {
        let done = Arc::clone(&done);
        let tx = tx.clone();
        pool.execute(move || {
            done.fetch_add(1, Ordering::SeqCst);
            tx.send(()).expect("notification impossible");
        });
    }

    drop(tx);

    for _ in 0..8 {
        rx.recv_timeout(Duration::from_secs(1))
            .expect("job non exécuté dans le délai");
    }

    assert_eq!(done.load(Ordering::SeqCst), 8);
}

/// Vérifie que la création avec 0 worker panique.
#[test]
#[should_panic(expected = "thread pool size must be greater than zero")]
fn thread_pool_new_panique_si_taille_zero() {
    let _ = ThreadPool::new(0);
}

/// Vérifie qu'un envoi après panic worker ne panique pas l'appelant.
#[test]
fn thread_pool_execute_apres_panic_worker() {
    let pool = ThreadPool::new(1);

    pool.execute(|| panic!("panic volontaire"));

    thread::sleep(Duration::from_millis(50));
    pool.execute(|| {});
}
