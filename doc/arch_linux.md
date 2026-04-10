# Installer Pixie Sur Arch Linux

Ce guide fournit un paquet Arch Linux (`pixie-git`) pour installer Pixie via `pacman`.

## Installation depuis AUR (recommande pour les utilisateurs)

Avec un helper AUR:

```bash
yay -S pixie-git
```

Ou manuellement:

```bash
git clone https://aur.archlinux.org/pixie-git.git
cd pixie-git
makepkg -si
```

## Prerequis

```bash
sudo pacman -Syu --needed base-devel git
```

## Installation via PKGBUILD local (mainteneur/dev)

```bash
git clone https://github.com/laflut3/Pixie.git
cd Pixie/arch
makepkg -si
```

Le paquet installe:
- `/usr/bin/pixie`
- `/usr/share/pixie/web/*.html`
- `/etc/pixie/config-pixie.yml`
- `/usr/lib/systemd/system/pixie.service`

## Demarrer Le Service

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now pixie.service
```

## Mettre A Jour

Depuis un clone local:

```bash
cd Pixie
git pull
cd arch
makepkg -si
```

## Configuration

Le fichier principal est:

`/etc/pixie/config-pixie.yml`

L'ordre de resolution de la config runtime est documente ici:
[doc/configuration.md](/home/ltorres/perso/Pixie/doc/configuration.md)

Pour publier/mettre a jour le paquet AUR:
[doc/release_all_channels.md](/home/ltorres/perso/Pixie/doc/release_all_channels.md)
