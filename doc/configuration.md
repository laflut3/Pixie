# Configuration Pixie

Ce document definit ou placer `config-pixie.yml` selon le mode de deploiement.

## Chemin standard (comme Nginx)

Pour une installation systeme (APT), le chemin standard est:

`/etc/pixie/config-pixie.yml`

Objectif:
- meme logique que `nginx` (fichiers de conf sous `/etc/...`),
- configuration separee du code applicatif,
- simple a gerer avec systemd et outils d'administration.

## Format du fichier

Exemple minimal:

```yml
host: 127.0.0.1
port: 8080
nb_worker: 4
```

Cles supportees:
- `host`
- `port`
- `nb_worker` (alias: `workers`)
- `addr` (prioritaire sur `host` + `port`, ex: `0.0.0.0:8080`)

## Ordre de resolution

Pixie lit la configuration dans cet ordre:
1. `PIXIE_CONFIG` (chemin explicite)
2. `/etc/pixie/config-pixie.yml`
3. `./config-pixie.yml`
4. Valeurs hardcodees dans le code (`127.0.0.1:80`, `4` workers)

## Docker: monter le YAML en volume

Le fichier doit etre monte dans le conteneur:

```bash
docker run --rm -p 8080:8080 \
  -v "$(pwd)/config-pixie.yml:/etc/pixie/config-pixie.yml:ro" \
  pixie:local
```

## Kubernetes: monter le YAML via ConfigMap/volume

Exemple de `volumeMount`:

```yaml
volumeMounts:
  - name: pixie-config
    mountPath: /etc/pixie/config-pixie.yml
    subPath: config-pixie.yml
    readOnly: true
volumes:
  - name: pixie-config
    configMap:
      name: pixie-config
```

## APT: emplacement attendu

Avec `apt install pixie`, le fichier de configuration doit etre gere sous:

`/etc/pixie/config-pixie.yml`

Puis appliquer la conf:

```bash
sudo systemctl restart pixie
```
