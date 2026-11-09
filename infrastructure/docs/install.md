# Install Pixie

Choose one method.

## Docker

```sh
docker run --rm -p 8080:8080 ghcr.io/laflut3/pixie:latest
```

## Kubernetes

```sh
helm upgrade --install pixie infrastructure/helm/pixie \
  --namespace pixie --create-namespace
```

## Debian / Ubuntu

```sh
curl -fsSL https://raw.githubusercontent.com/laflut3/Pixie/main/infrastructure/apt/install.sh | bash
sudo systemctl enable --now pixie
```

## Arch Linux

```sh
yay -S pixie-git
sudo systemctl enable --now pixie
```

## Build a local Debian package

```sh
infrastructure/scripts/build-deb.sh
sudo apt install ./pixie_*.deb
sudo systemctl enable --now pixie
```

## Check

```sh
curl http://localhost:8080/hello
```
