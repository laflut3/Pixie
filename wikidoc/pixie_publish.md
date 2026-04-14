# Publication Debian De Pixie

## Objectif

Publier `pixie` dans Debian via le workflow officiel:

- ITP (WNPP)
- upload mentors
- RFS (sponsorship-requests)
- sponsoring DD

## Prerequis

- Compte mentors.debian.net
- Cle GPG locale (publique importee sur mentors)
- Outils Debian:

```bash
sudo apt install -y reportbug devscripts dput-ng lintian
```

## Procedure

1. Creer l ITP:

```bash
reportbug wnpp
```

Type: `ITP`

2. Preparer le paquet source Debian:

- format source non-native (`3.0 (quilt)`)
- metadata Debian propres (`debian/control`, `debian/copyright`)

3. Construire le source upload:

```bash
debuild -S -sa
```

4. Signer:

```bash
debsign -k<KEYID> ../pixie_<VERSION>_source.changes
```

5. Uploader sur mentors:

```bash
dput mentors ../pixie_<VERSION>_source.changes
```

6. Creer/mettre a jour le RFS:

```bash
reportbug sponsorship-requests
```

Puis suivre le bug RFS (ex: `#1133771`) avec le lien `dget -x` mentors.

## Actions effectuees pour Pixie

- ITP: `#1133770`
- RFS: `#1133771`
- Upload mentors:
  `https://mentors.debian.net/debian/pool/main/p/pixie/pixie_1.1.0-2.dsc`

## Notes importantes

- Tant qu un DD ne sponsorise pas et que le paquet n est pas accepte,
  `apt install pixie` global n est pas encore garanti.
- Une fois accepte dans Debian, l installation devient standard via APT.
