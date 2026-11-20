#!/usr/bin/env bash
set -euo pipefail
REPO="${PIXIE_PACKAGE_REPOSITORY:-https://laflut3.github.io/Pixie}"
command -v pacman >/dev/null || { echo "Arch Linux is required" >&2; exit 1; }
command -v curl >/dev/null || { echo "curl is required" >&2; exit 1; }
command -v gpg >/dev/null || { echo "gpg is required" >&2; exit 1; }
run() { if [ "$(id -u)" -eq 0 ]; then "$@"; else sudo "$@"; fi; }
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
curl -fsSL --proto '=https' --tlsv1.2 "$REPO/pixie-archive-keyring.gpg" -o "$WORK/key.gpg"
FINGERPRINT="$(gpg --show-keys --with-colons "$WORK/key.gpg" | awk -F: '$1 == "fpr" {print $10; exit}')"
[ -n "$FINGERPRINT" ] || { echo "Signing key has no fingerprint" >&2; exit 1; }
run pacman-key --init
run pacman-key --add "$WORK/key.gpg"
run pacman-key --lsign-key "$FINGERPRINT"
if ! grep -Eq '^\[pixie\]$' /etc/pacman.conf; then
  printf '\n[pixie]\nSigLevel = Required DatabaseRequired\nServer = %s/arch/$arch\n' "$REPO" | run tee -a /etc/pacman.conf >/dev/null
fi
echo "Repository configured. Install with: sudo pacman -Syu pixie"
