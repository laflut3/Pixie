#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
REPO_DIR="${REPO_DIR:-$ROOT_DIR/apt-repo}"
DIST="${DIST:-bookworm}"
COMPONENT="${COMPONENT:-main}"
ARCH="${ARCH:-amd64}"
GPG_KEY_ID="${GPG_KEY_ID:-}"
BUILD_DEB="${BUILD_DEB:-1}"

if ! command -v dpkg-buildpackage >/dev/null 2>&1; then
  echo "dpkg-buildpackage is required. Install devscripts/dpkg-dev." >&2
  exit 1
fi

if ! command -v reprepro >/dev/null 2>&1; then
  echo "reprepro is required. Install it with: sudo apt install reprepro" >&2
  exit 1
fi

if [[ "$BUILD_DEB" == "1" ]]; then
  (
    cd "$ROOT_DIR"
    dpkg-buildpackage -us -uc -b
  )
fi

DEB_FILE="$(ls -t "$ROOT_DIR"/../pixie_*_"$ARCH".deb 2>/dev/null | head -n1 || true)"
if [[ -z "$DEB_FILE" ]]; then
  echo "No .deb package found for architecture '$ARCH' in $(dirname "$ROOT_DIR")." >&2
  echo "Build one first with: dpkg-buildpackage -us -uc -b" >&2
  exit 1
fi

mkdir -p "$REPO_DIR/conf"

if [[ ! -f "$REPO_DIR/conf/distributions" ]]; then
  {
    echo "Origin: Pixie"
    echo "Label: Pixie"
    echo "Suite: stable"
    echo "Codename: $DIST"
    echo "Architectures: $ARCH"
    echo "Components: $COMPONENT"
    echo "Description: Pixie APT repository"
    if [[ -n "$GPG_KEY_ID" ]]; then
      echo "SignWith: $GPG_KEY_ID"
    fi
  } >"$REPO_DIR/conf/distributions"
fi

if [[ -n "$GPG_KEY_ID" ]]; then
  if ! gpg --list-keys "$GPG_KEY_ID" >/dev/null 2>&1; then
    echo "GPG key '$GPG_KEY_ID' not found in local keyring." >&2
    exit 1
  fi
fi

reprepro -b "$REPO_DIR" includedeb "$DIST" "$DEB_FILE"

if [[ -n "$GPG_KEY_ID" ]]; then
  mkdir -p "$REPO_DIR/keyrings"
  TMP_ASC="$(mktemp)"
  gpg --armor --export "$GPG_KEY_ID" >"$TMP_ASC"
  cp "$TMP_ASC" "$REPO_DIR/keyrings/pixie-archive-keyring.asc"
  gpg --dearmor --yes --output "$REPO_DIR/keyrings/pixie-archive-keyring.gpg" "$TMP_ASC"
  rm -f "$TMP_ASC"
fi

cat <<EOF
Repository updated in: $REPO_DIR
Package added: $DEB_FILE

Next steps:
1. Host '$REPO_DIR' over HTTPS.
2. On client machine:
   sudo install -d -m 0755 /usr/share/keyrings
   curl -fsSL <REPO_URL>/keyrings/pixie-archive-keyring.gpg | sudo tee /usr/share/keyrings/pixie-archive-keyring.gpg >/dev/null
   echo "deb [signed-by=/usr/share/keyrings/pixie-archive-keyring.gpg] <REPO_URL> $DIST $COMPONENT" | sudo tee /etc/apt/sources.list.d/pixie.list
   sudo apt update
   sudo apt install pixie
EOF
