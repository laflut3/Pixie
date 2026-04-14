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

Publication en cours dans les depots officiels Debian.

Quand le paquet sera disponible dans la distribution cible:

```bash
sudo apt update
sudo apt install -y pixie
```

Suivi publication Debian: `ITP #1133770`, `RFS #1133771`.

### Arch Linux

```bash
yay -S pixie-git
```

Le service est gere automatiquement par le systeme d'installation.

## Securite du depot public

- Ne jamais commiter de cle privee, token, mot de passe ou fichier `.env`.
- Les artefacts de build/packaging (`*.deb`, `*.dsc`, `*.changes`, `*.buildinfo`, `*.orig.tar.*`) sont ignores par `.gitignore`.
- Les secrets CI/CD doivent rester dans GitHub Actions Secrets, jamais dans le code.
