mod common;

use std::{thread, time::Duration};

use common::{EnvGuard, TempDir, env_lock, reserve_local_port, send_http_request, wait_for_server};
use pixie::run_server;

/// Vérifie une réponse 200 sur `/` puis une réponse 404 sur une page absente.
#[test]
fn serveur_repond_200_et_404() {
    let _lock = env_lock();

    let port = reserve_local_port();
    let addr = format!("127.0.0.1:{port}");

    let web_root = TempDir::new("pixie-server-web");
    web_root.write("hello.html", "Bonjour depuis test");
    web_root.write("404.html", "Page absente");

    let _web_env = EnvGuard::set(
        "PIXIE_WEB_ROOT",
        web_root.path.to_str().expect("chemin web non UTF-8"),
    );

    let addr_for_thread = addr.clone();
    thread::spawn(move || {
        let _ = run_server(&addr_for_thread, 1);
    });

    wait_for_server(&addr, Duration::from_secs(3));

    let response_ok = send_http_request(&addr, "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
        .expect("requête 200 impossible");
    assert!(response_ok.starts_with("HTTP/1.1 200 OK"));
    assert!(response_ok.contains("Bonjour depuis test"));

    let response_404 =
        send_http_request(&addr, "GET /inconnue HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .expect("requête 404 impossible");
    assert!(response_404.starts_with("HTTP/1.1 404 NOT FOUND"));
    assert!(response_404.contains("Page absente"));
}

/// Vérifie que `run_server` remonte une erreur de bind sur adresse invalide.
#[test]
fn run_server_remonte_une_erreur_si_l_adresse_est_invalide() {
    let err = run_server("256.0.0.1:80", 1).expect_err("une erreur de bind était attendue");
    assert!(!err.to_string().is_empty());
}

/// Vérifie que le serveur gère l'erreur interne quand `404.html` est absent.
#[test]
fn serveur_journalise_une_erreur_si_404_est_absent() {
    let _lock = env_lock();

    let port = reserve_local_port();
    let addr = format!("127.0.0.1:{port}");

    let web_root = TempDir::new("pixie-server-web-no-404");
    web_root.write("hello.html", "Bonjour depuis test");

    let _web_env = EnvGuard::set(
        "PIXIE_WEB_ROOT",
        web_root.path.to_str().expect("chemin web non UTF-8"),
    );

    let addr_for_thread = addr.clone();
    thread::spawn(move || {
        let _ = run_server(&addr_for_thread, 1);
    });

    wait_for_server(&addr, Duration::from_secs(3));

    let response_404 =
        send_http_request(&addr, "GET /inconnue HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .expect("requête impossible");

    assert!(response_404.is_empty());
}
