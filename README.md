<div align="center">
  <img src="docs/assets/mark.svg" width="64" height="64" alt="Pixie">
  <h1>Pixie</h1>
  <p><strong>Deploy your static HTML server.</strong></p>
  <p>
    <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-d9471a?style=flat-square">
    <img alt="HTTP" src="https://img.shields.io/badge/HTTP-1.0%20%7C%201.1%20%7C%202-1764a8?style=flat-square">
    <img alt="HTTP/3" src="https://img.shields.io/badge/HTTP%2F3-experimental-7952b3?style=flat-square">
    <img alt="HTTPS" src="https://img.shields.io/badge/HTTPS-TLS%20%7C%20mTLS-168279?style=flat-square">
  </p>
  <p><a href="docs/index.html"><strong>📖 Documentation</strong></a> · <a href="docs/publish.html">Host the docs</a></p>
</div>

## 🚀 Docker

```sh
docker run --rm -p 8080:8080 ghcr.io/laflut3/pixie:latest
```

Open **http://localhost:8080/**.

## 📦 Deployment options

Run repository commands from a local checkout.

| Method | Start here | Guide |
| :--- | :--- | :--- |
| 🐳 **Docker Compose** | `docker compose -f infrastructure/docker/compose.yaml up -d` | [Containers](docs/deploy.html#docker) |
| ☸ **Kubernetes / Helm** | `helm upgrade --install pixie infrastructure/helm/pixie --namespace pixie --create-namespace` | [Kubernetes](docs/deploy.html#kubernetes) |
| 🐧 **Debian / Ubuntu** | Use the signed APT repository installer below. | [Linux packages](docs/deploy.html#linux) |
| 📐 **Arch Linux** | `yay -S pixie-git` | [Linux packages](docs/deploy.html#linux) |
| 🦀 **Cargo** | `PIXIE_CONFIG=infrastructure/config/config-pixie.yml cargo run --locked` | [Local build](docs/deploy.html#cargo) |
| 🛠 **Local Debian package** | `infrastructure/scripts/build-deb.sh` | [Build and install](docs/deploy.html#linux) |

### Debian / Ubuntu installer

Download and review the installer, then run it:

```sh
curl -fsSLo install-pixie.sh https://raw.githubusercontent.com/laflut3/Pixie/main/infrastructure/apt/install.sh
less install-pixie.sh
bash install-pixie.sh
sudo systemctl enable --now pixie
```
