# Publier Pixie Pour Tous Les Canaux

Ce guide mainteneur publie Pixie sur:
- image Docker (GHCR)
- Debian (APT)
- Arch Linux (AUR)

## 1. Verifier Que Le Serveur N'est Plus Local-Only

Avant toute publication, verifie que la config serveur ecoute hors loopback:

```yml
addr: 0.0.0.0:8080
workers: 4
```

Chemin conseille en production:

`/etc/pixie/config-pixie.yml`

Verification:

```bash
sudo systemctl restart pixie
ss -ltnp | grep 8080
```

## 2. Publier Docker (GHCR)

Prerequis:
- le repo GitHub est public
- GitHub Actions activees

Etapes:
1. Creer un tag semver.
2. Pousser branche + tag.
3. Laisser le workflow Docker publier l'image.

Commandes:

```bash
git tag v1.1.0
git push origin main --tags
```

Workflow utilise:

`/.github/workflows/docker-image.yml`

Image finale:

`ghcr.io/laflut3/pixie:latest`

## 3. Publier Debian (APT)

Etapes:
1. Mettre a jour `debian/changelog`.
2. Construire et signer via le script de publication.
3. Sync le dossier `apt-repo/` sur ton hebergement HTTPS.

Commandes:

```bash
GPG_KEY_ID=<KEY_ID> DIST=bookworm COMPONENT=main ./scripts/apt/publish-repo.sh
```

Puis publier `apt-repo/` sur:

`https://repo.example.org/pixie`

Doc detaillee:

`/doc/publish_repo.md`

## 4. Publier Arch Linux (AUR)

Le package est defini dans:

`/arch/PKGBUILD`

Etapes:
1. Generer le metadata AUR.
2. Pousser `PKGBUILD` + `.SRCINFO` dans le repo AUR `pixie-git`.

Commandes (depuis `arch/`):

```bash
makepkg --printsrcinfo > .SRCINFO
```

Exemple de publication AUR:

```bash
git clone ssh://aur@aur.archlinux.org/pixie-git.git
cp PKGBUILD .SRCINFO pixie-git/
cd pixie-git
git add PKGBUILD .SRCINFO
git commit -m "pixie-git: update"
git push
```

## 5. Checklist De Release

1. `cargo test --locked` vert.
2. Tag Git cree et pousse.
3. Image GHCR visible.
4. Repo APT synchronise et signe.
5. AUR mis a jour (`PKGBUILD` + `.SRCINFO`).
