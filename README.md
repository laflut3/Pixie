# Pixie

Pixie est un serveur HTTP statique ecrit en Rust.

Version actuelle: `1.1.0`

## Installation

### Docker

```bash
docker pull ghcr.io/laflut3/pixie:latest
docker run --rm -p 8080:8080 ghcr.io/laflut3/pixie:latest
```

### Debian / Ubuntu

```bash
./scripts/apt/configure-client.sh https://repo.example.org/pixie bookworm main
sudo apt update
sudo apt install -y pixie
```

Le service est gere automatiquement par le systeme d'installation.

### Arch Linux

```bash
yay -S pixie-git
```

Le service est gere automatiquement par le systeme d'installation.
