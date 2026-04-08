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
sudo journalctl -u pixie.service -n 100 --no-pager
sudo journalctl -u pixie.service -f
```

Option (sans `sudo`, si ton user est dans `adm` ou `systemd-journal`):

```bash
pixie log -q
pixie log -q -f
```

## Deploiement Docker

Construire l'image localement:

```bash
docker build -t pixie:local .
docker run --rm -p 8080:8080 pixie:local
```

Puis ouvrir:

`http://localhost:8080`

Image publiee (GitHub Container Registry):

`ghcr.io/<owner>/<repo>:latest`

Utiliser une image publiee:

```bash
docker pull ghcr.io/<owner>/<repo>:latest
docker run --rm -p 8080:8080 ghcr.io/<owner>/<repo>:latest
```

Variables utiles:
- `PIXIE_ADDR` (defaut image Docker: `0.0.0.0:8080`)
- `PIXIE_THREADS` (defaut: `4`)
