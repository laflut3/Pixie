# Pixie

Pixie is a lightweight static HTTP server written in Rust. It serves the HTML files in `web/`. The sample deployment configuration listens on `0.0.0.0:8080`.

Pixie currently supports HTTP/1.0 and HTTP/1.1. HTTPS can be enabled with TLS certificates, with optional client-certificate verification (mTLS); see [TLS configuration](infrastructure/docs/tls.md). HTTP/2 and HTTP/3 are not implemented yet. HTTP/1.2 and HTTP/1.3 are not standardized protocol versions.

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

HTTPS can be enabled with `tls.cert_file` and `tls.key_file` in the config file. Set `tls.client_ca_file` to require client certificates (mTLS). See [TLS configuration](infrastructure/docs/tls.md).

## Project layout

- `src/`: Rust library and binary
- `tests/`: integration tests
- `web/`: static pages served by Pixie
- `infrastructure/`: deployment, packaging, and infrastructure documentation

See [CONTRIBUTING.md](CONTRIBUTING.md) for development checks.
