#!/usr/bin/env bash
set -euo pipefail
REPO="https://github.com/laflut3/Pixie"
VERSION="${PIXIE_VERSION:-latest}"
PREFIX=""
FROM_SOURCE=false
usage() { echo "Usage: install.sh [--version vX.Y.Z] [--prefix directory] [--source]"; }
while [ "$#" -gt 0 ]; do
  case "$1" in
    --version) VERSION="${2:?Specify a version}"; shift 2 ;;
    --prefix) PREFIX="${2:?Specify an installation directory}"; shift 2 ;;
    --source) FROM_SOURCE=true; shift ;;
    --help|-h) usage; exit 0 ;;
    *) usage >&2; exit 1 ;;
  esac
done
[ "$(uname -s)" = Linux ] || { echo "Linux is required" >&2; exit 1; }
case "$(uname -m)" in x86_64|amd64) ARCH=amd64 ;; aarch64|arm64) ARCH=arm64 ;; *) echo "Unsupported architecture" >&2; exit 1 ;; esac
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT
if "$FROM_SOURCE"; then
  ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
  [ -f "$ROOT/Cargo.toml" ] || { echo "Run --source from a repository checkout" >&2; exit 1; }
  (cd "$ROOT" && cargo build --release --locked)
  "$ROOT/infrastructure/scripts/build-archive.sh" "$ROOT/target/release/pixie" "$ARCH" "$WORK"
  ARCHIVE="$(find "$WORK" -maxdepth 1 -name '*.tar.gz' -print -quit)"
else
  command -v curl >/dev/null || { echo "curl is required" >&2; exit 1; }
  command -v sha256sum >/dev/null || { echo "sha256sum is required" >&2; exit 1; }
  if [ "$VERSION" = latest ]; then
    RELEASE_URL="$(curl -fsSL --proto '=https' --tlsv1.2 -o /dev/null -w '%{url_effective}' "$REPO/releases/latest")"
    VERSION="${RELEASE_URL##*/}"
  fi
  [[ "$VERSION" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || { echo "Use a stable version such as v1.2.0" >&2; exit 1; }
  NAME="pixie_${VERSION#v}_linux_${ARCH}.tar.gz"
  URL="$REPO/releases/download/$VERSION"
  curl -fsSL --proto '=https' --tlsv1.2 "$URL/$NAME" -o "$WORK/$NAME"
  curl -fsSL --proto '=https' --tlsv1.2 "$URL/SHA256SUMS" -o "$WORK/SHA256SUMS"
  awk -v name="$NAME" '$2 == name || $2 == "*" name { print; found=1 } END { if (!found) exit 1 }' "$WORK/SHA256SUMS" > "$WORK/selected.sha256"
  (cd "$WORK" && sha256sum --check selected.sha256)
  ARCHIVE="$WORK/$NAME"
fi
mkdir "$WORK/payload"
tar -xzf "$ARCHIVE" -C "$WORK/payload" usr/bin/pixie etc/pixie/config-pixie.yml etc/pixie/sites/default usr/share/pixie/web usr/lib/systemd/system/pixie.service usr/share/licenses/pixie/LICENSE
run() {
  if [ "$(id -u)" -eq 0 ] || [ -n "$PREFIX" ]; then "$@"; else sudo "$@"; fi
}
run install -Dm755 "$WORK/payload/usr/bin/pixie" "$PREFIX/usr/bin/pixie"
run install -Dm644 "$WORK/payload/usr/lib/systemd/system/pixie.service" "$PREFIX/usr/lib/systemd/system/pixie.service"
run install -Dm644 "$WORK/payload/usr/share/licenses/pixie/LICENSE" "$PREFIX/usr/share/licenses/pixie/LICENSE"
if [ ! -e "$PREFIX/etc/pixie/config-pixie.yml" ]; then
  run install -Dm644 "$WORK/payload/etc/pixie/config-pixie.yml" "$PREFIX/etc/pixie/config-pixie.yml"
fi
run install -d "$PREFIX/usr/share/pixie/web"
for PAGE in "$WORK/payload/usr/share/pixie/web/"*.html; do
  if [ ! -e "$PREFIX/usr/share/pixie/web/$(basename "$PAGE")" ]; then
    run install -m644 "$PAGE" "$PREFIX/usr/share/pixie/web/"
  fi
done
run install -d "$PREFIX/etc/pixie/sites/default"
for PAGE in "$WORK/payload/etc/pixie/sites/default/"*.html; do
  if [ ! -e "$PREFIX/etc/pixie/sites/default/$(basename "$PAGE")" ]; then
    run install -m644 "$PAGE" "$PREFIX/etc/pixie/sites/default/"
  fi
done
if [ -z "$PREFIX" ] && command -v systemctl >/dev/null && [ -d /run/systemd/system ]; then
  run systemctl daemon-reload
fi
echo "Pixie installed. Config: $PREFIX/etc/pixie/config-pixie.yml"
echo "Start the service with: sudo systemctl enable --now pixie"
