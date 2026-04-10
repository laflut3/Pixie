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

## Installation Arch Linux

```bash
yay -S pixie-git
```

Activer le service:

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now pixie.service
```

Guide detaille:
[doc/arch_linux.md](/home/ltorres/perso/Pixie/doc/arch_linux.md)

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
addr: 0.0.0.0:8080
workers: 4
```

Champs supportes:
- `addr` (prioritaire sur `host` + `port`, ex: `0.0.0.0:8080`)
- `host` (ex: `127.0.0.1`)
- `port` (ex: `8080`)
- `workers` (alias accepte: `nb_worker`)

Ordre de resolution:
1. `PIXIE_CONFIG` (chemin de fichier explicite)
2. `/etc/pixie/config-pixie.yml`
3. `./config-pixie.yml`
4. Valeurs hardcodees dans le code (`127.0.0.1:80`, `4` workers)

Guide complet (Docker, Kubernetes, APT):
[doc/configuration.md](/home/ltorres/perso/Pixie/doc/configuration.md)

Publication multi-canaux (Docker + Debian + Arch/AUR):
[doc/release_all_channels.md](/home/ltorres/perso/Pixie/doc/release_all_channels.md)

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

Variable utile:
- `PIXIE_CONFIG` (chemin explicite vers un fichier YAML de configuration)
