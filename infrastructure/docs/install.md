# Install Pixie

Install without cloning the repository.

## Docker

```sh
docker run --rm -p 8080:8080 ghcr.io/laflut3/pixie:latest
```

## Kubernetes

```sh
helm repo add pixie https://laflut3.github.io/Pixie/charts
helm repo update
helm upgrade --install pixie pixie/pixie --namespace pixie --create-namespace
```

## Debian / Ubuntu

Install `curl` and `gnupg` first if they are missing.

```sh
curl -fsSL https://raw.githubusercontent.com/laflut3/Pixie/main/infrastructure/apt/install.sh -o install-pixie-apt.sh
bash install-pixie-apt.sh --setup-only
sudo apt install pixie
sudo systemctl enable --now pixie
```

## Arch Linux

```sh
curl -fsSL https://raw.githubusercontent.com/laflut3/Pixie/main/infrastructure/arch/install.sh -o install-pixie-arch.sh
bash install-pixie-arch.sh
sudo pacman -Syu pixie
sudo systemctl enable --now pixie
```

## Linux binary

The installer selects amd64 or arm64, verifies `SHA256SUMS`, and preserves existing config and pages. It needs curl and tar. Review the script before running it.

```sh
curl -fsSL https://raw.githubusercontent.com/laflut3/Pixie/main/infrastructure/scripts/install.sh -o install-pixie.sh
bash install-pixie.sh
sudo systemctl enable --now pixie
```

Pin a release with `bash install-pixie.sh --version v1.2.0`. The prebuilt archives use static musl binaries.

## Serve a static site

```sh
sudo mkdir -p /etc/pixie/sites/default
sudo cp -R ./dist/. /etc/pixie/sites/default/
sudo systemctl restart pixie
```

## React single-page app

Set `PIXIE_SPA_FALLBACK=true` in the Pixie service environment.

## Check

```sh
curl http://localhost:8080/
```

Config: `/etc/pixie/config-pixie.yml`. Sites: `/etc/pixie/sites/default` or `/etc/pixie/sites/<hostname>`. APT, Arch, Helm, and binary downloads are published when a matching SemVer tag is pushed. Each patch, feature, or breaking release gets a matching `vX.Y.Z` tag.
