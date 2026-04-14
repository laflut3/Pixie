# Installation Arch Linux

## Installation Utilisateur (AUR)

Avec helper AUR:

```bash
yay -S pixie-git
```

Le service est gere automatiquement par le systeme d'installation.

## Configuration

Fichier principal:

`/etc/pixie/config-pixie.yml`

Exemple expose reseau:

```yml
addr: 0.0.0.0:8080
workers: 4
```

Note:

- `pixie-git` est le nom du paquet AUR.
- Le binaire installe s'appelle `pixie`.
