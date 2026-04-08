# Pixie

Serveur HTTP en Rust.

## Installation rapide (machine cliente)

1. Configurer le depot APT Pixie:

```bash
./scripts/apt/configure-client.sh https://repo.example.org/pixie bookworm main
```

`repo.example.org` est un exemple. Remplace-le par ton vrai domaine.

2. Installer Pixie:

```bash
sudo apt update
sudo apt install -y pixie
```

Le service `pixie` est demarre automatiquement et active au boot.

## Mettre a jour Pixie

```bash
sudo apt update
sudo apt install --only-upgrade -y pixie
```

## Verifier le service et les logs

```bash
sudo systemctl status pixie
pixie log
pixie log -f
```
