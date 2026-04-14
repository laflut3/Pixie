# Publication Du Contenu

Ce document explique comment publier Pixie sur les 3 canaux:

- Docker (GHCR)
- Debian (APT)
- Arch Linux (AUR)

## Prerequis

- acces mainteneur au repo GitHub
- GitHub Actions active
- cle GPG pour signer le repo APT
- acces au repo AUR `pixie-git`

## 1. Docker (GHCR)

Le workflow de build/push est:

`/.github/workflows/docker-image.yml`

Publication:

```bash
git tag v1.1.0
git push origin main --tags
```

Resultat attendu:

- `ghcr.io/laflut3/pixie:latest`
- image multi-arch `linux/amd64` et `linux/arm64`

## 2. Debian (APT)

Le script de publication est:

`/scripts/apt/publish-repo.sh`

Commande:

```bash
GPG_KEY_ID=<KEY_ID> DIST=bookworm COMPONENT=main ./scripts/apt/publish-repo.sh
```

Ensuite publier `apt-repo/` sur ton endpoint HTTPS (ex: `https://repo.example.org/pixie`).

Installation client standard (sans script):

```bash
sudo install -d -m 0755 /usr/share/keyrings
curl -fsSL https://repo.example.org/pixie/keyrings/pixie-archive-keyring.gpg \
  | sudo tee /usr/share/keyrings/pixie-archive-keyring.gpg >/dev/null
echo "deb [signed-by=/usr/share/keyrings/pixie-archive-keyring.gpg] https://repo.example.org/pixie bookworm main" \
  | sudo tee /etc/apt/sources.list.d/pixie.list >/dev/null
sudo apt update
sudo apt install -y pixie
```

## 3. Arch Linux (AUR)

Le package source est:

`/arch/PKGBUILD`

Generer le metadata AUR:

```bash
cd arch
makepkg --printsrcinfo > .SRCINFO
```

Puis pousser `PKGBUILD` et `.SRCINFO` dans le repo AUR `pixie-git`.

## Verification Rapide

- Docker: `docker pull ghcr.io/laflut3/pixie:latest`
- Debian: `apt-cache policy pixie`
- Arch: verifier la version AUR `pixie-git`
