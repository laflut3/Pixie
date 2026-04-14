# Installation Debian

## Installation

1. Configurer le repo APT Pixie:

```bash
sudo install -d -m 0755 /usr/share/keyrings
curl -fsSL https://repo.example.org/pixie/keyrings/pixie-archive-keyring.gpg \
  | sudo tee /usr/share/keyrings/pixie-archive-keyring.gpg >/dev/null
echo "deb [signed-by=/usr/share/keyrings/pixie-archive-keyring.gpg] https://repo.example.org/pixie bookworm main" \
  | sudo tee /etc/apt/sources.list.d/pixie.list >/dev/null
```

2. Installer Pixie:

```bash
sudo apt update
sudo apt install -y pixie
```

Le service est gere automatiquement par le systeme d'installation.

## Configuration

Fichier principal:

`/etc/pixie/config-pixie.yml`

Exemple expose reseau:

```yml
addr: 0.0.0.0:8080
workers: 4
```
