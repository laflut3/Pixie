# Installation Debian

## Installation

1. Configurer le repo APT Pixie:

```bash
./scripts/apt/configure-client.sh https://repo.example.org/pixie bookworm main
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
