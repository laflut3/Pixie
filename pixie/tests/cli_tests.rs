use std::process::Command;

/// Vérifie le code de sortie et le message quand la commande est inconnue.
#[test]
fn cli_rejette_une_commande_inconnue() {
    let output = Command::new(env!("CARGO_BIN_EXE_pixie"))
        .arg("build")
        .output()
        .expect("exécution du binaire impossible");

    assert_eq!(output.status.code(), Some(2));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unknown command: build"));
}
