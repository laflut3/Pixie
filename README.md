# Pixie

Serveur HTTP en Rust.

## Branch name

Allowed branch :
feat/...
fix/...
docs/...
chore/...
test/...
refactor/...
ci/...
build/...
perf/...
style/...

## Objectif

Pixie est un projet OPEN SOURCE de création d'un server HTTP en Rust:

- demarrer d'un serveur TCP brut en Rust;
- parser des requetes HTTP manuellement;
- router les routes vers des handlers;
- renvoyer des reponses HTTP propres (status, headers, body);
- faire evoluer le socle vers une architecture propre et testable.

## Stack technique

- Langage: Rust (edition 2024)
- Runtime: standard library (`std::net::TcpListener`)
- Build tool: Cargo

## Prerequis

- Rust installe (via `rustup`)
- Cargo disponible dans le PATH

Verifier:

```bash
rustc --version
cargo --version
```

## Lancer le serveur

Depuis la racine du repo:

```bash
cd pixie
cargo run
```

Note: le serveur bind `127.0.0.1:80`. Sur Linux, ce port est privilegie
et demande des droits supplementaires (voir section bare-metal plus bas).

## Installation Bare-Metal (Debian/Ubuntu)

Le projet contient un packaging Debian dans `debian/`.

### 1) Installer les dependances de build

```bash
sudo apt update
sudo apt install -y build-essential debhelper devscripts cargo rustc
```

### 2) Construire le paquet `.deb`

Depuis la racine du repo (`Pixie/`):

```bash
dpkg-buildpackage -us -uc -b
```

Le paquet genere sera dans le dossier parent, par exemple:

```bash
../pixie_0.1.0-1_amd64.deb
```

### 3) Installer le paquet

```bash
sudo apt install -y ../pixie_0.1.0-1_amd64.deb
```

### 4) Verifier les fichiers installes

```bash
dpkg -L pixie
```

Le binaire est installe dans `/usr/bin/pixie` et les pages web dans
`/usr/share/pixie/web`.

### 5) Autoriser le bind sur le port 80 (sans lancer en root)

```bash
sudo apt install -y libcap2-bin
sudo setcap 'cap_net_bind_service=+ep' /usr/bin/pixie
```

Puis lancer:

```bash
/usr/bin/pixie
```

Note: si un cluster local (ex: k3s) redirige deja le port 80 via iptables/nft,
les requetes HTTP peuvent etre captees avant d'arriver a Pixie.

## Installation Via `apt install pixie` (repository APT)

Pour installer sur n'importe quelle machine Debian/Ubuntu avec `apt install pixie`,
il faut publier un repository APT signe.

### 1) Construire et publier le repository (machine publisher)

Prerequis:

```bash
sudo apt update
sudo apt install -y build-essential debhelper devscripts rustc cargo reprepro gnupg
```

Generer une cle GPG dediee au repo (une seule fois):

```bash
gpg --full-generate-key
gpg --list-keys --keyid-format LONG
```

Publier le paquet dans un repo local (`apt-repo/`):

```bash
GPG_KEY_ID=<KEY_ID> DIST=bookworm COMPONENT=main ./scripts/apt/publish-repo.sh
```

Ensuite, heberger le dossier `apt-repo/` en HTTPS (Nginx, S3, GitHub Pages, etc.).

### 2) Installer depuis une machine cliente

Avec le script fourni:

```bash
./scripts/apt/configure-client.sh https://packages.example.com/pixie bookworm main
```

Ou manuellement:

```bash
sudo install -d -m 0755 /usr/share/keyrings
curl -fsSL https://packages.example.com/pixie/keyrings/pixie-archive-keyring.gpg \
  | sudo tee /usr/share/keyrings/pixie-archive-keyring.gpg >/dev/null
echo "deb [signed-by=/usr/share/keyrings/pixie-archive-keyring.gpg] https://packages.example.com/pixie bookworm main" \
  | sudo tee /etc/apt/sources.list.d/pixie.list
sudo apt update
sudo apt install -y pixie
```

## Licence

Ce projet est distribue sous licence MIT. Voir le fichier `LICENSE`.
