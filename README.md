# Pixie

Serveur HTTP en Rust.

## Installation 1: paquet `.deb` local (bare-metal Debian/Ubuntu)

Depuis la racine du repo (`Pixie/`):

```bash
sudo apt update
sudo apt install -y build-essential debhelper devscripts cargo rustc
dpkg-buildpackage -us -uc -b
sudo apt install -y ../pixie_0.1.0-1_amd64.deb
```

Verifier l'installation:

```bash
dpkg -L pixie
```

Donner le droit de bind sur `:80` sans root:

```bash
sudo apt install -y libcap2-bin
sudo setcap 'cap_net_bind_service=+ep' /usr/bin/pixie
```

## Installation 2: `apt install pixie` via depot APT

### 2.1 Publier le depot (machine publisher)

```bash
sudo apt update
sudo apt install -y build-essential debhelper devscripts rustc cargo reprepro gnupg
gpg --full-generate-key
gpg --list-keys --keyid-format LONG
GPG_KEY_ID=<KEY_ID> DIST=bookworm COMPONENT=main ./scripts/apt/publish-repo.sh
```

Puis heberger le dossier `apt-repo/` en HTTPS.

### 2.2 Installer depuis une machine cliente

Avec le script:

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
