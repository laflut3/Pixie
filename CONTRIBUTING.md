# Contributing to Pixie

## Requirements

- Git
- Rust and Cargo (installable with `rustup`)

Check your toolchain:

```sh
rustc --version
cargo --version
```

## Repository layout

- `src/`: Rust library and binary
- `tests/`: integration tests
- `web/`: static pages served by Pixie
- `infrastructure/`: Docker, Helm, Debian/APT, Arch packaging, scripts, and deployment documentation

## Local checks

Run from the repository root:

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --all-features --locked
```

At the repository root, run configured hooks with:

```sh
pre-commit run --all-files
```

To build a local Debian package, install Cargo and `dpkg-deb`, then run:

```sh
infrastructure/scripts/build-deb.sh
```

## Pull requests

Use a descriptive branch name such as `feat/...`, `fix/...`, `docs/...`, or `infrastructure/...`. Keep each pull request focused, document user-visible changes, and do not commit credentials, private keys, tokens, `.env` files, or generated packages.
