# Pixie

Serveur HTTP en Rust.

## Objectif

Pixie est un projet OPEN SOURCE de création d'un server HTTP en Rust:

- demarrer d'un serveur TCP brut en Rust;
- parser des requetes HTTP manuellement;
- router les routes vers des handlers;
- renvoyer des reponses HTTP propres (status, headers, body);
- faire evoluer le socle vers une architecture propre et testable.

## Stack technique

- Langage: Rust (edition 2024)
- Runtime: standard library (`std::net::TcpListener`)
- Build tool: Cargo

## Prerequis

- Rust installe (via `rustup`)
- Cargo disponible dans le PATH

Verifier:

```bash
rustc --version
cargo --version
```

## Lancer le serveur

Depuis la racine du repo:

```bash
cd pixie
cargo run
```

## Licence

Ce projet est distribue sous licence MIT. Voir le fichier `LICENSE`.
