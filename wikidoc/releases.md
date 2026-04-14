# Releases

Ce document definit la procedure de release Pixie.

## Version

La version applicative vient de:

`/pixie/Cargo.toml`

## Regles

- utiliser un tag semver: `vMAJOR.MINOR.PATCH`
- mettre a jour les changelogs avant tag
- verifier que la CI est verte

## Procedure

1. verifier les tests:

```bash
cd pixie
cargo test --locked
```

2. creer le tag:

```bash
git tag v1.1.0
git push origin main --tags
```

3. lancer la publication des canaux:

- Docker via workflow GitHub Actions
- Debian via `scripts/apt/publish-repo.sh`
- Arch via mise a jour `PKGBUILD` + `.SRCINFO` sur AUR

Installation Debian cliente recommandee (sans script):

```bash
sudo install -d -m 0755 /usr/share/keyrings
curl -fsSL https://repo.example.org/pixie/keyrings/pixie-archive-keyring.gpg \
  | sudo tee /usr/share/keyrings/pixie-archive-keyring.gpg >/dev/null
echo "deb [signed-by=/usr/share/keyrings/pixie-archive-keyring.gpg] https://repo.example.org/pixie bookworm main" \
  | sudo tee /etc/apt/sources.list.d/pixie.list >/dev/null
sudo apt update
sudo apt install -y pixie
```

## Checklist Release

- [ ] version code validee
- [ ] tests locaux OK
- [ ] tag pousse
- [ ] image Docker disponible
- [ ] repo APT mis a jour
- [ ] AUR mis a jour
