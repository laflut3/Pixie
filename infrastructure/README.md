# Pixie infrastructure

This directory contains Pixie's deployment and packaging assets.

## Contents

- `docker/`: Docker image definition and Compose example
- `helm/pixie/`: Kubernetes Helm chart
- `apt/`: signed APT repository generation and installation scripts
- `debian/`: Debian package metadata, service unit, and man page
- `arch/`: Arch Linux AUR packaging
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

Build a local Debian package with `infrastructure/scripts/build-deb.sh`. Publishing the public APT repository is automated by `.github/workflows/publish-apt.yml` when a version tag such as `v1.1.0` is pushed. Configure the repository's `APT_GPG_PRIVATE_KEY` Actions secret and `APT_GPG_KEY_ID` Actions variable and (for a passphrase-protected key) the `APT_GPG_PASSPHRASE` Actions secret, and enable GitHub Pages with the GitHub Actions deployment source before the first release.
