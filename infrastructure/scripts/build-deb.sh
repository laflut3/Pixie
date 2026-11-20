#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VERSION="$(sed -n 's/^version = "\([^"]*\)"/\1/p' "$ROOT/Cargo.toml" | head -n1)"
ARCH="$(dpkg --print-architecture)"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
PKG="$WORK/pixie"
BINARY="${1:-$ROOT/target/release/pixie}"
if [ "$#" -eq 0 ]; then
  (cd "$ROOT" && cargo build --release --locked)
fi
install -Dm755 "$BINARY" "$PKG/usr/bin/pixie"
install -Dm644 "$ROOT/infrastructure/config/config-pixie.yml" "$PKG/etc/pixie/config-pixie.yml"
install -Dm644 "$ROOT/infrastructure/debian/pixie.service" "$PKG/lib/systemd/system/pixie.service"
install -Dm644 "$ROOT/infrastructure/debian/pixie.1" "$PKG/usr/share/man/man1/pixie.1"
install -d "$PKG/usr/share/pixie/web"
install -m644 "$ROOT"/web/*.html "$PKG/usr/share/pixie/web/"
install -d "$PKG/etc/pixie/sites/default"
install -m644 "$ROOT"/web/*.html "$PKG/etc/pixie/sites/default/"
install -Dm644 "$ROOT/LICENSE" "$PKG/usr/share/doc/pixie/copyright"
mkdir -p "$PKG/DEBIAN"
cat > "$PKG/DEBIAN/control" <<EOF
Package: pixie
Version: ${VERSION}
Section: web
Priority: optional
Architecture: ${ARCH}
Maintainer: Pixie Team <leo0609leo@gmail.com>
Description: Minimal static HTTP server written in Rust
 Pixie serves static HTML pages over HTTP.
EOF
printf '/etc/pixie/config-pixie.yml\n/etc/pixie/sites/default/hello.html\n/etc/pixie/sites/default/404.html\n' > "$PKG/DEBIAN/conffiles"
dpkg-deb --root-owner-group --build "$PKG" "$ROOT/pixie_${VERSION}_${ARCH}.deb"
printf 'Built %s\n' "$ROOT/pixie_${VERSION}_${ARCH}.deb"
