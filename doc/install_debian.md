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

## Service

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now pixie.service
sudo systemctl status pixie
```

## Configuration

Fichier principal:

`/etc/pixie/config-pixie.yml`

Exemple expose reseau:

```yml
addr: 0.0.0.0:8080
workers: 4
```

## Mise A Jour

```bash
sudo apt update
sudo apt install --only-upgrade -y pixie
```
