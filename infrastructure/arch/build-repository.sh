#!/usr/bin/env bash
set -euo pipefail
PACKAGES="${1:?Usage: build-repository.sh <packages-dir> <arch-output-dir>}"
OUTPUT="${2:?Specify an output directory, e.g. public/arch}"
: "${APT_GPG_PRIVATE_KEY:?Set APT_GPG_PRIVATE_KEY}"
command -v repo-add >/dev/null || { echo "repo-add is required (Arch pacman package)" >&2; exit 1; }
command -v bsdtar >/dev/null || { echo "bsdtar is required (Arch libarchive package)" >&2; exit 1; }
command -v gpg >/dev/null || { echo "gpg is required" >&2; exit 1; }
GNUPGHOME="$(mktemp -d)"
export GNUPGHOME
chmod 700 "$GNUPGHOME"
trap 'rm -rf "$GNUPGHOME"' EXIT
printf '%s' "$APT_GPG_PRIVATE_KEY" | gpg --batch --import
KEY="${APT_GPG_KEY_ID:-$(gpg --batch --list-secret-keys --with-colons | awk -F: '$1 == "sec" {print $5; exit}')}"
[ -n "$KEY" ] || { echo "Signing key not found" >&2; exit 1; }
printf '%s' "${APT_GPG_PASSPHRASE:-}" > "$GNUPGHOME/passphrase"
sign() {
  gpg --batch --yes --pinentry-mode loopback --passphrase-file "$GNUPGHOME/passphrase" --local-user "$KEY" --detach-sign "$1"
}
for ARCH in x86_64 aarch64; do
  DIR="$OUTPUT/$ARCH"
  mkdir -p "$DIR"
  shopt -s nullglob
  FILES=("$PACKAGES"/pixie-*-"$ARCH".pkg.tar.zst)
  [ "${#FILES[@]}" -gt 0 ] || { echo "No $ARCH packages found" >&2; exit 1; }
  for FILE in "${FILES[@]}"; do
    cp "$FILE" "$DIR/"
    sign "$DIR/$(basename "$FILE")"
  done
  repo-add "$DIR/pixie.db.tar.gz" "$DIR"/*.pkg.tar.zst
  sign "$DIR/pixie.db.tar.gz"
  sign "$DIR/pixie.files.tar.gz"
  ln -sf pixie.db.tar.gz.sig "$DIR/pixie.db.sig"
  ln -sf pixie.files.tar.gz.sig "$DIR/pixie.files.sig"
done
echo "Signed Arch repositories generated in $OUTPUT"
