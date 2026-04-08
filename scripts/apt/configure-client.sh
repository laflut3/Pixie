#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 || $# -gt 3 ]]; then
  echo "Usage: $0 <repo-url> [distribution] [component]" >&2
  echo "Example: $0 https://packages.example.com/pixie bookworm main" >&2
  exit 1
fi

REPO_URL="${1%/}"
DIST="${2:-bookworm}"
COMPONENT="${3:-main}"
KEYRING_PATH="/usr/share/keyrings/pixie-archive-keyring.gpg"
LIST_PATH="/etc/apt/sources.list.d/pixie.list"

if ! command -v curl >/dev/null 2>&1; then
  echo "curl is required." >&2
  exit 1
fi

if ! command -v sudo >/dev/null 2>&1; then
  echo "sudo is required." >&2
  exit 1
fi

sudo install -d -m 0755 /usr/share/keyrings
curl -fsSL "$REPO_URL/keyrings/pixie-archive-keyring.gpg" | sudo tee "$KEYRING_PATH" >/dev/null

echo "deb [signed-by=$KEYRING_PATH] $REPO_URL $DIST $COMPONENT" | sudo tee "$LIST_PATH" >/dev/null

sudo apt update
sudo apt install -y pixie
