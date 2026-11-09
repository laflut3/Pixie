# Docker

## Run the published image

```sh
docker run --rm -p 8080:8080 ghcr.io/laflut3/pixie:latest
```

## Build locally

```sh
docker build -f infrastructure/docker/Dockerfile -t pixie:local .
docker run --rm -p 8080:8080 pixie:local
```

## Docker Compose

```sh
docker compose -f infrastructure/docker/compose.yaml up -d
curl http://localhost:8080/hello
docker compose -f infrastructure/docker/compose.yaml down
```

The image supports `amd64` and `arm64`; Docker selects the matching version automatically.
