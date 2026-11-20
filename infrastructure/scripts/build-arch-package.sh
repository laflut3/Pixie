#!/usr/bin/env bash
set -euo pipefail
ARCHIVE="${1:?Usage: build-arch-package.sh <linux-archive> <output-dir>}"
OUTPUT="${2:?Specify an output directory}"
NAME="$(basename "$ARCHIVE")"
case "$NAME" in
  pixie_*_linux_amd64.tar.gz) ARCH=x86_64 ;;
  pixie_*_linux_arm64.tar.gz) ARCH=aarch64 ;;
  *) echo "Unsupported archive name: $NAME" >&2; exit 1 ;;
esac
VERSION="${NAME#pixie_}"
VERSION="${VERSION%_linux_*}"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
tar -xzf "$ARCHIVE" -C "$WORK"
cat > "$WORK/.PKGINFO" <<EOF
pkgname = pixie
pkgbase = pixie
pkgver = ${VERSION}-1
pkgdesc = Minimal static HTTP server written in Rust
url = https://github.com/laflut3/Pixie
builddate = ${SOURCE_DATE_EPOCH:-$(date +%s)}
packager = Pixie Team
size = $(du -sb "$WORK" | cut -f1)
arch = ${ARCH}
license = MIT
backup = etc/pixie/config-pixie.yml
backup = etc/pixie/sites/default/hello.html
backup = etc/pixie/sites/default/404.html
EOF
mkdir -p "$OUTPUT"
tar --owner=0 --group=0 --zstd -cf "$OUTPUT/pixie-${VERSION}-1-${ARCH}.pkg.tar.zst" -C "$WORK" .PKGINFO usr etc
