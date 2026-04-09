# Publier Le Depot APT Pixie

Ce guide explique comment publier une nouvelle version de `pixie` dans le depot APT, puis comment les clients recuperent la mise a jour.

## Prerequis

```bash
sudo apt update
sudo apt install -y build-essential debhelper devscripts rustc cargo reprepro gnupg
```

## 1. Preparation d'une nouvelle version

1. Mettre a jour le code de `pixie`.
2. Mettre a jour `debian/changelog` avec une nouvelle version Debian (ex: `0.1.0-3`).

Exemple de verification:

```bash
head -n 20 debian/changelog
```

## 2. Creer et publier le paquet dans le repo

1. Recuperer l'ID de ta cle GPG:

```bash
gpg --list-keys --keyid-format LONG
```

2. Publier:

```bash
GPG_KEY_ID=<KEY_ID> DIST=bookworm COMPONENT=main ./scripts/apt/publish-repo.sh
```

Ce script:
- build le `.deb`,
- ajoute le paquet dans `apt-repo/`,
- signe les metadonnees du depot,
- met a jour `apt-repo/keyrings/pixie-archive-keyring.gpg`.

## 3. Mettre en ligne le dossier `apt-repo/`

Publier le contenu de `apt-repo/` sur ton serveur HTTP(S), par exemple sur:

`https://repo.example.org/pixie`

Important:
- en production, preferer HTTPS,
- pour un usage local avec `file:`, eviter un chemin sous `/home` (preferer `/srv/pixie-apt`).

## 4. Cote client: installation initiale

```bash
./scripts/apt/configure-client.sh https://repo.example.org/pixie bookworm main
```

Le script configure la cle, ajoute la source APT, installe `pixie` et active le service.

Configuration apres installation (comme nginx sous `/etc`):

`/etc/pixie/config-pixie.yml`

## 5. Cote client: recuperer une mise a jour Pixie

Quand une nouvelle version est publiee:

```bash
sudo apt update
sudo apt install --only-upgrade -y pixie
```

Verifier:

```bash
sudo systemctl status pixie
pixie log -f
```

## 6. Checks utiles en cas de probleme

Verifier la cle publiee:

```bash
gpg --show-keys --with-fingerprint apt-repo/keyrings/pixie-archive-keyring.asc
```

Verifier la version dispo dans le repo:

```bash
apt-cache policy pixie
```
