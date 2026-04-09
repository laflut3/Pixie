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

## Configuration Pixie (`config-pixie.yml`)

Chemin recommande en installation systeme (meme logique que nginx):

`/etc/pixie/config-pixie.yml`

Pour le developpement local, `./config-pixie.yml` est aussi supporte.

Exemple:

```yml
host: 127.0.0.1
port: 80
nb_worker: 4
```

Champs supportes:
- `host` (ex: `127.0.0.1`)
- `port` (ex: `8080`)
- `nb_worker` (alias: `workers`)
- `addr` (optionnel, prioritaire sur `host` + `port`, ex: `0.0.0.0:8080`)

Ordre de resolution:
1. `PIXIE_CONFIG` (chemin de fichier explicite)
2. `/etc/pixie/config-pixie.yml`
3. `./config-pixie.yml`
4. Variables d'environnement (`PIXIE_ADDR`, `PIXIE_THREADS`)
5. Valeurs par defaut

Guide complet (Docker, Kubernetes, APT):
[doc/configuration.md](/home/ltorres/perso/Pixie/doc/configuration.md)

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
