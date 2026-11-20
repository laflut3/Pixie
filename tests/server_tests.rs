mod common;

use std::{thread, time::Duration};

use common::{EnvGuard, TempDir, env_lock, reserve_local_port, send_http_request, wait_for_server};
use pixie::run_server;

/// Verifies a 200 response for `/` and a 404 response for a missing page.
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
        .expect("HTTP 200 request failed");
    assert!(response_ok.starts_with("HTTP/1.1 200 OK"));
    assert!(response_ok.contains("Bonjour depuis test"));

    let response_404 =
        send_http_request(&addr, "GET /inconnue HTTP/1.1\r\nHost: localhost\r\n\r\n")
            .expect("HTTP 404 request failed");
    assert!(response_404.starts_with("HTTP/1.1 404 NOT FOUND"));
    assert!(response_404.contains("Page absente"));
}

/// Verifies that `run_server` returns a bind error for an invalid address.
#[test]
fn run_server_remonte_une_erreur_si_l_adresse_est_invalide() {
    let err = run_server("256.0.0.1:80", 1).expect_err("expected a bind error");
    assert!(!err.to_string().is_empty());
}

/// Verifies a valid HTTP 404 response when `404.html` is missing.
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
            .expect("request failed");

    assert!(response_404.starts_with("HTTP/1.1 404 NOT FOUND"));
    assert!(response_404.contains("Not Found"));
}
