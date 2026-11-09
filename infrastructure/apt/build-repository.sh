#!/usr/bin/env bash
set -euo pipefail
PACKAGES_DIR="${1:?Usage: build-repository.sh <packages-dir> <output-dir>}"
OUTPUT_DIR="${2:?Usage: build-repository.sh <packages-dir> <output-dir>}"
: "${APT_GPG_PRIVATE_KEY:?Set APT_GPG_PRIVATE_KEY to the ASCII-armored private signing key}"
command -v apt-ftparchive >/dev/null || { echo "apt-ftparchive is required (install apt-utils)" >&2; exit 1; }
command -v gpg >/dev/null || { echo "gpg is required" >&2; exit 1; }
if [ -e "$OUTPUT_DIR" ] && [ -n "$(find "$OUTPUT_DIR" -mindepth 1 -maxdepth 1 -print -quit)" ]; then
  echo "Output directory must be empty: $OUTPUT_DIR" >&2
  exit 1
fi
mkdir -p "$OUTPUT_DIR/pool/main/p/pixie" "$OUTPUT_DIR/dists/stable/main"
if ! compgen -G "$PACKAGES_DIR/*.deb" >/dev/null; then
  echo "No Debian packages found in $PACKAGES_DIR" >&2
  exit 1
fi
cp "$PACKAGES_DIR"/*.deb "$OUTPUT_DIR/pool/main/p/pixie/"
GNUPGHOME="$(mktemp -d)"
chmod 700 "$GNUPGHOME"
export GNUPGHOME
trap 'rm -rf "$GNUPGHOME"' EXIT
printf '%s' "$APT_GPG_PRIVATE_KEY" | gpg --batch --import
if [ -n "${APT_GPG_KEY_ID:-}" ]; then
  KEY_ID="$APT_GPG_KEY_ID"
else
  KEY_ID="$(gpg --batch --list-secret-keys --with-colons | awk -F: '$1 == "sec" {print $5; exit}')"
fi
[ -n "$KEY_ID" ] || { echo "Could not determine the signing key ID" >&2; exit 1; }
PASSPHRASE_FILE="$(mktemp)"
chmod 600 "$PASSPHRASE_FILE"
printf '%s' "${APT_GPG_PASSPHRASE:-}" > "$PASSPHRASE_FILE"
trap 'rm -rf "$GNUPGHOME"; rm -f "$PASSPHRASE_FILE"' EXIT
for ARCH in amd64 arm64; do
  DIR="$OUTPUT_DIR/dists/stable/main/binary-$ARCH"
  mkdir -p "$DIR"
  (cd "$OUTPUT_DIR" && apt-ftparchive packages pool/main/p/pixie | awk -v arch="$ARCH" 'BEGIN { RS=""; ORS="\n\n" } { ok=0; n=split($0, lines, "\n"); for (i=1; i<=n; i++) if (lines[i] == "Architecture: " arch) ok=1; if (ok) print $0 }' > "$DIR/Packages")
  [ -s "$DIR/Packages" ] || { echo "No $ARCH packages found in $PACKAGES_DIR" >&2; exit 1; }
  gzip -9n -c "$DIR/Packages" > "$DIR/Packages.gz"
done
cat > "$OUTPUT_DIR/apt-release.conf" <<'CONF'
APT::FTPArchive::Release::Origin "Pixie";
APT::FTPArchive::Release::Label "Pixie APT Repository";
APT::FTPArchive::Release::Suite "stable";
APT::FTPArchive::Release::Codename "stable";
APT::FTPArchive::Release::Architectures "amd64 arm64";
APT::FTPArchive::Release::Components "main";
APT::FTPArchive::Release::Description "Pixie static HTTP server packages";
CONF
(cd "$OUTPUT_DIR" && apt-ftparchive -c apt-release.conf release dists/stable > dists/stable/Release)
gpg --batch --yes --pinentry-mode loopback --passphrase-file "$PASSPHRASE_FILE" --local-user "$KEY_ID" --armor --detach-sign -o "$OUTPUT_DIR/dists/stable/Release.gpg" "$OUTPUT_DIR/dists/stable/Release"
gpg --batch --yes --pinentry-mode loopback --passphrase-file "$PASSPHRASE_FILE" --local-user "$KEY_ID" --clearsign -o "$OUTPUT_DIR/dists/stable/InRelease" "$OUTPUT_DIR/dists/stable/Release"
gpg --batch --export "$KEY_ID" > "$OUTPUT_DIR/pixie-archive-keyring.gpg"
rm "$OUTPUT_DIR/apt-release.conf"
printf 'Signed APT repository generated in %s\n' "$OUTPUT_DIR"
