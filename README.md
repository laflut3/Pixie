# Pixie

Pixie is a lightweight static HTTP server written in Rust. It serves the HTML files in `web/`. The sample deployment configuration listens on `0.0.0.0:8080`.

## Install

[Quick installation commands](infrastructure/docs/install.md)

## Deployment guides

- [Docker and Compose](infrastructure/docs/docker.md)
- [Kubernetes with Helm](infrastructure/docs/kubernetes.md)
- [APT repository](infrastructure/docs/apt.md)
- [Install with curl](infrastructure/docs/curl.md)
- [Build a Debian package](infrastructure/docs/debian-package.md)
- [Arch Linux](infrastructure/docs/arch.md)

## Development

```sh
PIXIE_CONFIG=infrastructure/config/config-pixie.yml cargo run --locked
```

## Project layout

- `src/`: Rust library and binary
- `tests/`: integration tests
- `web/`: static pages served by Pixie
- `infrastructure/`: deployment, packaging, and infrastructure documentation

See [CONTRIBUTING.md](CONTRIBUTING.md) for development checks.
