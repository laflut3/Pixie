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
sudo install -d -m 0755 /usr/share/keyrings
curl -fsSL https://repo.example.org/pixie/keyrings/pixie-archive-keyring.gpg \
  | sudo tee /usr/share/keyrings/pixie-archive-keyring.gpg >/dev/null
echo "deb [signed-by=/usr/share/keyrings/pixie-archive-keyring.gpg] https://repo.example.org/pixie bookworm main" \
  | sudo tee /etc/apt/sources.list.d/pixie.list >/dev/null
sudo apt update
sudo apt install -y pixie
```

Le service est gere automatiquement par le systeme d'installation.

### Arch Linux

```bash
yay -S pixie-git
```

Le service est gere automatiquement par le systeme d'installation.
