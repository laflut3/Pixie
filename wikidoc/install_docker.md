# Installation Docker

## Option 1 - Docker CLI

Pull de l'image:

```bash
docker pull ghcr.io/laflut3/pixie:latest
```

Execution simple:

```bash
docker run --rm -p 8080:8080 ghcr.io/laflut3/pixie:latest
```

Execution avec fichier de configuration local:

```bash
docker run --rm -p 8080:8080 \
  -v "$(pwd)/config-pixie.yml:/etc/pixie/config-pixie.yml:ro" \
  ghcr.io/laflut3/pixie:latest
```

## Option 2 - Docker Compose

Exemple `compose.yml`:

```yaml
services:
  pixie:
    image: ghcr.io/laflut3/pixie:latest
    container_name: pixie
    ports:
      - "8080:8080"
    volumes:
      - ./config-pixie.yml:/etc/pixie/config-pixie.yml:ro
    restart: unless-stopped
```

Lancer:

```bash
docker compose up -d
```

Arreter:

```bash
docker compose down
```

## Verification

```bash
curl http://localhost:8080/hello
```
