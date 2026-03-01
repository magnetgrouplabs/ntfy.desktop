#!/usr/bin/env bash
# Usage: ./scripts/bump-version.sh <new-version>
# Example: ./scripts/bump-version.sh 26.3.2
#
# Updates version in all three files:
#   - package.json
#   - src-tauri/Cargo.toml
#   - src-tauri/tauri.conf.json

set -euo pipefail

if [ $# -ne 1 ]; then
  echo "Usage: $0 <new-version>"
  echo "Example: $0 26.3.2"
  exit 1
fi

NEW_VERSION="$1"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

# Validate version format (digits and dots)
if ! echo "$NEW_VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
  echo "Error: Version must be in format X.Y.Z (e.g., 26.3.1)"
  exit 1
fi

echo "Bumping version to $NEW_VERSION..."

# 1. package.json
sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$NEW_VERSION\"/" "$ROOT_DIR/package.json"
echo "  Updated package.json"

# 2. src-tauri/Cargo.toml (only the package version, not dependency versions)
sed -i "0,/^version = \"[^\"]*\"/s//version = \"$NEW_VERSION\"/" "$ROOT_DIR/src-tauri/Cargo.toml"
echo "  Updated src-tauri/Cargo.toml"

# 3. src-tauri/tauri.conf.json
sed -i "s/\"version\": \"[^\"]*\"/\"version\": \"$NEW_VERSION\"/" "$ROOT_DIR/src-tauri/tauri.conf.json"
echo "  Updated src-tauri/tauri.conf.json"

echo ""
echo "Version bumped to $NEW_VERSION in all files."
echo "Run 'cargo check --manifest-path src-tauri/Cargo.toml' to verify."
