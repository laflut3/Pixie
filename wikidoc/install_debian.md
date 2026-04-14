# Installation Debian / Ubuntu

## Etat actuel

La publication officielle Debian est en cours (`ITP #1133770`, `RFS #1133771`).

Tant que le paquet n est pas accepte dans les depots de la distribution cible,
la commande `apt install pixie` ne fonctionnera pas partout par defaut.

## Installation officielle (cible)

Quand `pixie` est present dans les depots de ta distribution:

```bash
sudo apt update
sudo apt install -y pixie
```

## Installation de test (avant disponibilite officielle)

Depuis mentors.debian.net:

```bash
sudo apt install -y devscripts
cd /tmp
dget -x https://mentors.debian.net/debian/pool/main/p/pixie/pixie_1.1.0-3.dsc
cd pixie-1.1.0
dpkg-buildpackage -us -uc -b
sudo apt install ../pixie_1.1.0-3_amd64.deb
```

## Configuration

Fichier principal:

`/etc/pixie/config-pixie.yml`

Exemple:

```yml
addr: 0.0.0.0:8080
workers: 4
```
