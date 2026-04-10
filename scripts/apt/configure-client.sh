#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 || $# -gt 3 ]]; then
  echo "Usage: $0 <repo-url> [distribution] [component]" >&2
  echo "Example: $0 https://repo.example.org/pixie bookworm main" >&2
  echo "Example: $0 file:/srv/pixie-apt bookworm main" >&2
  exit 1
fi

REPO_URL="${1%/}"
DIST="${2:-bookworm}"
COMPONENT="${3:-main}"
KEYRING_PATH="/usr/share/keyrings/pixie-archive-keyring.gpg"
LIST_PATH="/etc/apt/sources.list.d/pixie.list"
TMP_KEYRING="$(mktemp)"
trap 'rm -f "$TMP_KEYRING"' EXIT

if ! command -v sudo >/dev/null 2>&1; then
  echo "sudo is required." >&2
  exit 1
fi

if [[ "$REPO_URL" == "https://packages.example.com/pixie" ]]; then
  echo "Replace packages.example.com with your real repository URL." >&2
  exit 1
fi

case "$REPO_URL" in
  http://*|https://*)
    if ! command -v curl >/dev/null 2>&1; then
      echo "curl is required for http(s) repositories." >&2
      exit 1
    fi
    curl -fsSL "$REPO_URL/keyrings/pixie-archive-keyring.gpg" -o "$TMP_KEYRING"
    ;;
  file:*)
    # file:/path/to/repo -> /path/to/repo
    LOCAL_REPO_PATH="${REPO_URL#file:}"
    cp "$LOCAL_REPO_PATH/keyrings/pixie-archive-keyring.gpg" "$TMP_KEYRING"
    ;;
  *)
    echo "Unsupported repo URL: '$REPO_URL'" >&2
    echo "Use an http(s) URL or file:/absolute/path URL." >&2
    exit 1
    ;;
esac

if [[ ! -s "$TMP_KEYRING" ]]; then
  echo "Downloaded keyring is empty: $TMP_KEYRING" >&2
  exit 1
fi

if command -v gpg >/dev/null 2>&1; then
  if ! gpg --show-keys "$TMP_KEYRING" >/dev/null 2>&1; then
    echo "Downloaded keyring is not a valid OpenPGP keyring." >&2
    exit 1
  fi
fi

sudo install -d -m 0755 /usr/share/keyrings
sudo install -m 0644 "$TMP_KEYRING" "$KEYRING_PATH"

# Remove stale pixie entries to avoid duplicate/broken sources.
sudo rm -f /etc/apt/sources.list.d/pixie-local.list

echo "deb [signed-by=$KEYRING_PATH] $REPO_URL $DIST $COMPONENT" | sudo tee "$LIST_PATH" >/dev/null

sudo apt update
sudo apt install -y pixie

if command -v systemctl >/dev/null 2>&1; then
  sudo systemctl enable --now pixie
fi

cat <<EOF
Pixie installed.
- Service status: sudo systemctl status pixie
- Follow logs:   sudo journalctl -u pixie.service -f
- Upgrade later: sudo apt update && sudo apt install --only-upgrade pixie
EOF
