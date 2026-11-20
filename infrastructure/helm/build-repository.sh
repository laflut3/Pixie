#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 2 ]]; then
  echo "Usage: $0 VERSION OUTPUT_DIR" >&2
  exit 2
fi

VERSION="$1"
OUTPUT_DIR="$2"
REPOSITORY_URL="https://laflut3.github.io/Pixie/charts"
RELEASE_URL="https://github.com/laflut3/Pixie/releases/download/v${VERSION}"
INDEX_URL="${REPOSITORY_URL}/index.yaml"
PREVIOUS_INDEX="$(mktemp)"
trap 'rm -f "$PREVIOUS_INDEX"' EXIT

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CHART_DIR="${ROOT}/infrastructure/helm/pixie"

mkdir -p "$OUTPUT_DIR"
helm package "$CHART_DIR" --version "$VERSION" --app-version "$VERSION" --destination "$OUTPUT_DIR"

HTTP_STATUS="$(curl --silent --show-error --location --output "$PREVIOUS_INDEX" --write-out '%{http_code}' "$INDEX_URL")"
case "$HTTP_STATUS" in
  200) helm repo index "$OUTPUT_DIR" --url "$RELEASE_URL" --merge "$PREVIOUS_INDEX" ;;
  404) helm repo index "$OUTPUT_DIR" --url "$RELEASE_URL" ;;
  *) echo "Could not load the existing Helm index: HTTP $HTTP_STATUS" >&2; exit 1 ;;
esac
