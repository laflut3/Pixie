# Contributing to Pixie

Merci de contribuer a Pixie.

Ce document explique le workflow attendu pour proposer des changements.

## Prerequis

- Git
- Rust (via `rustup`)
- Cargo

Verification rapide:

```bash
rustc --version
cargo --version
```

## Structure du projet

- `pixie/`: code Rust (librairie + binaire)
- `web/`: pages HTML servies par le serveur
- `debian/`: packaging Debian

## Nommage des branches

Utiliser une branche avec un prefixe autorise:

- `feat/...`
- `fix/...`
- `docs/...`
- `chore/...`
- `test/...`
- `refactor/...`
- `ci/...`
- `build/...`
- `perf/...`
- `style/...`

## Workflow de contribution

1. Mettre `main` a jour.
2. Creer une branche de travail.
3. Faire les changements par commits coherents.
4. Lancer les checks locaux.
5. Ouvrir une Pull Request.

Exemple:

```bash
git checkout main
git pull
git checkout -b feat/ma-feature
```

## Checks locaux obligatoires

Depuis `pixie/`:

```bash
cargo fmt --all
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --all-features --locked
```

## Pre-commit

A la racine du repo:

```bash
pre-commit install
pre-commit run --all-files
```

Le hook `pre-commit` execute automatiquement:

- `cargo fmt --all -- --check` (dans `pixie/`)
- `cargo clippy --locked --all-targets --all-features -- -D warnings` (dans `pixie/`)

## Si vous modifiez le packaging Debian

Depuis la racine:

```bash
dpkg-buildpackage -us -uc -b
```

## Pull Request checklist

Avant de soumettre:

- [ ] La branche respecte le format de nommage.
- [ ] `cargo fmt`, `cargo clippy` et `cargo test` passent en local.
- [ ] Les changements sont documentes si necessaire (`README.md`).
- [ ] Aucun secret/cle privee/fichier sensible n est commit.
- [ ] La CI GitHub est verte.

## Hygiene securite (repo public)

- Ne pas commiter de secrets (`token`, `password`, cle privee, `.env`).
- Ne pas commiter d artefacts de packaging (`*.deb`, `*.dsc`, `*.changes`, `*.buildinfo`, `*.orig.tar.*`).
- Verifier rapidement avant push:

```bash
git status --short
git diff --cached
```

## Style de commit recommande

- Messages clairs et courts, a l imperatif.
- Un commit = une intention logique.

Exemples:

- `feat(threadpool): split worker into dedicated module`
- `fix(http): handle missing file with 404 page`
- `docs(readme): add apt repository installation steps`
