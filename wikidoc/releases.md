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

## Checklist Release

- [ ] version code validee
- [ ] tests locaux OK
- [ ] tag pousse
- [ ] image Docker disponible
- [ ] repo APT mis a jour
- [ ] AUR mis a jour
