# Pixie infrastructure

This directory contains Pixie's deployment and packaging assets.

## Contents

- `docker/`: Docker image definition and Compose example
- `helm/pixie/`: Kubernetes Helm chart
- `apt/`: signed APT repository generation and installation scripts
- `debian/`: Debian package metadata, service unit, and man page
- `arch/`: signed Arch package repository and installer
- `config/`: default runtime configuration installed by packages and images
- `scripts/`: local package build helpers
- `docs/install.md`: quick install commands
- `docs/`: deployment guides and APT publishing instructions

## Supported CPU architectures

| Deployment format | x86-64 | ARM 64-bit |
| --- | --- | --- |
| Docker image | `linux/amd64` | `linux/arm64` |
| APT package | `amd64` | `arm64` |
| Arch package | `x86_64` | `aarch64` |
| Helm | Uses the matching platform from the multi-architecture image | Uses the matching platform from the multi-architecture image |

Docker selects the matching image platform automatically. APT and the curl installer support Debian/Ubuntu `amd64` and `arm64`. The Arch PKGBUILD supports `x86_64` and `aarch64` and builds natively on the target system.

The Docker build context is the repository root:

```sh
docker build -f infrastructure/docker/Dockerfile -t pixie:local .
```

Create each stable release by updating `Cargo.toml`, `Cargo.lock`, and `infrastructure/helm/pixie/Chart.yaml`, then pushing a matching `vX.Y.Z` tag. The release workflow publishes Docker tags, Linux packages, signed APT and Arch repositories, a Helm chart, and release notes. Configure `APT_GPG_PRIVATE_KEY` as a repository Actions secret; `APT_GPG_PASSPHRASE` is optional. Enable GitHub Pages with the GitHub Actions deployment source before the first release. Use SemVer: patch for fixes, minor for compatible features, and major for breaking changes.
