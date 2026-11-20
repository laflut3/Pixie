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

## From a source checkout

```sh
infrastructure/scripts/install.sh --source
sudo systemctl enable --now pixie
```

Or build a Debian package:

```sh
infrastructure/scripts/build-deb.sh
sudo apt install ./pixie_*.deb
sudo systemctl enable --now pixie
```

## Check

```sh
curl http://localhost:8080/
```

Config lives in `/etc/pixie/config-pixie.yml`; pages live in `/usr/share/pixie/web`. APT, Arch, and binary downloads become available after the release workflow publishes them. The third-party Arch repository is separate from Arch's official repositories. The source-based AUR package remains available as `pixie-git`.
