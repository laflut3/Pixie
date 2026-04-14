# Mise A Jour Rapide De Pixie (Debian)

## But

Mettre a jour rapidement `pixie` vers une nouvelle version Debian.

## Etapes minimales

1. Mettre a jour la version applicative:

- `pixie/Cargo.toml`

2. Ajouter une entree changelog Debian:

```bash
dch -i
```

3. Construire le source package:

```bash
debuild -S -sa
```

4. Signer:

```bash
debsign -k<KEYID> ../pixie_<VERSION>_source.changes
```

5. Upload mentors:

```bash
dput mentors ../pixie_<VERSION>_source.changes
```

6. Mettre a jour le bug RFS (`#1133771`) avec:

- nouvelle URL `dget -x`
- resume des changements

## Checklist ultra-courte

- [ ] version code ok
- [ ] `debian/changelog` incrementee
- [ ] `debuild -S -sa` passe
- [ ] `dput mentors` passe
- [ ] follow-up poste sur `#1133771`
