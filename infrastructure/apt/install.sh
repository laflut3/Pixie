#!/usr/bin/env bash
set -euo pipefail
REPO="${PIXIE_APT_REPOSITORY:-https://laflut3.github.io/Pixie}"
KEY_URL="${PIXIE_APT_KEY_URL:-$REPO/pixie-archive-keyring.gpg}"
command -v curl >/dev/null || { echo "curl is required" >&2; exit 1; }
command -v gpg >/dev/null || { echo "gpg is required" >&2; exit 1; }
command -v apt-get >/dev/null || { echo "This installer supports Debian and Ubuntu" >&2; exit 1; }
KEYRING=/usr/share/keyrings/pixie-archive-keyring.gpg
SOURCES=/etc/apt/sources.list.d/pixie.sources
TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT
curl -fsSL "$KEY_URL" -o "$TMP"
gpg --show-keys "$TMP" >/dev/null
sudo install -m 0644 "$TMP" "$KEYRING"
printf 'Types: deb\nURIs: %s\nSuites: stable\nComponents: main\nArchitectures: amd64 arm64\nSigned-By: %s\n' "$REPO" "$KEYRING" | sudo tee "$SOURCES" >/dev/null
sudo apt-get update
sudo apt-get install -y pixie
printf 'Pixie installed. Enable it with: sudo systemctl enable --now pixie\n'
