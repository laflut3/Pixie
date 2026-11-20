<div align="center">
  <img src="docs/assets/mark.svg" width="64" height="64" alt="Pixie">
  <h1>Pixie</h1>
  <p><strong>A small static HTTP server.</strong></p>
  <p>
    <img alt="Rust" src="https://img.shields.io/badge/Rust-2024-d9471a?style=flat-square">
    <img alt="HTTP" src="https://img.shields.io/badge/HTTP-1.0%20%7C%201.1%20%7C%202-1764a8?style=flat-square">
    <img alt="HTTP/3" src="https://img.shields.io/badge/HTTP%2F3-experimental-7952b3?style=flat-square">
    <img alt="HTTPS" src="https://img.shields.io/badge/HTTPS-TLS%20%7C%20mTLS-168279?style=flat-square">
  </p>
  <p><a href="docs/index.html"><strong>📖 Documentation</strong></a> · <a href="https://github.com/laflut3/Pixie/releases">Releases</a></p>
</div>

## Docker

```sh
docker run --rm -p 8080:8080 ghcr.io/laflut3/pixie:latest
```

## Install

```sh
helm repo add pixie https://laflut3.github.io/Pixie/charts
helm upgrade --install pixie pixie/pixie --namespace pixie --create-namespace
```

```sh
curl -fsSL https://raw.githubusercontent.com/laflut3/Pixie/main/infrastructure/apt/install.sh -o install-pixie.sh
bash install-pixie.sh --setup-only
sudo apt install pixie
```

```sh
curl -fsSL https://raw.githubusercontent.com/laflut3/Pixie/main/infrastructure/arch/install.sh -o install-pixie.sh
bash install-pixie.sh
sudo pacman -Syu pixie
```

```sh
curl -fsSL https://raw.githubusercontent.com/laflut3/Pixie/main/infrastructure/scripts/install.sh | sh
```

📚 [Deployment commands and configuration](docs/index.html)
