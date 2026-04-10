# Installation Arch Linux

## Installation Utilisateur (AUR)

Avec helper AUR:

```bash
yay -S pixie-git
```

Sans helper:

```bash
git clone https://aur.archlinux.org/pixie-git.git
cd pixie-git
makepkg -si
```

## Service

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now pixie.service
sudo systemctl status pixie
```

## Configuration

Fichier principal:

`/etc/pixie/config-pixie.yml`

Exemple expose reseau:

```yml
addr: 0.0.0.0:8080
workers: 4
```

## Mise A Jour

```bash
yay -Syu pixie-git
```

Note:

- `pixie-git` est le nom du paquet AUR.
- Le binaire installe s'appelle `pixie`.
