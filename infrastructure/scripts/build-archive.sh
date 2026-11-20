#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BINARY="${1:?Usage: build-archive.sh <binary> <amd64|arm64> <output-dir>}"
ARCH="${2:?Specify amd64 or arm64}"
OUTPUT="${3:?Specify an output directory}"
case "$ARCH" in amd64|arm64) ;; *) echo "Unsupported architecture: $ARCH" >&2; exit 1 ;; esac
VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$ROOT/Cargo.toml" | head -n1)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
install -Dm755 "$BINARY" "$WORK/usr/bin/pixie"
install -Dm644 "$ROOT/infrastructure/config/config-pixie.yml" "$WORK/etc/pixie/config-pixie.yml"
install -Dm644 "$ROOT/infrastructure/debian/pixie.service" "$WORK/usr/lib/systemd/system/pixie.service"
install -d "$WORK/usr/share/pixie/web"
install -m644 "$ROOT"/web/*.html "$WORK/usr/share/pixie/web/"
install -Dm644 "$ROOT/LICENSE" "$WORK/usr/share/licenses/pixie/LICENSE"
mkdir -p "$OUTPUT"
tar --owner=0 --group=0 -czf "$OUTPUT/pixie_${VERSION}_linux_${ARCH}.tar.gz" -C "$WORK" usr etc
