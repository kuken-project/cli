#!/bin/bash

set -e

REPO="kuken-project/cli"
INSTALL_DIR="$HOME/.local/bin"

echo "Installing Kuken CLI..."

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "$ARCH" in
    x86_64) ARCH="amd64" ;;
    aarch64|arm64) ARCH="arm64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

BINARY_NAME="kuken-${OS}-${ARCH}"

RELEASE_URL="https://api.github.com/repos/$REPO/releases/latest"
DOWNLOAD_URL=$(curl -s "$RELEASE_URL" | grep "browser_download_url.*$BINARY_NAME\"" | cut -d '"' -f 4)

if [ -z "$DOWNLOAD_URL" ]; then
    echo "Error: Binary not found for your system ($BINARY_NAME)"
    exit 1
fi

mkdir -p "$INSTALL_DIR"

echo "Downloading $BINARY_NAME..."
curl -fsSL "$DOWNLOAD_URL" -o "$INSTALL_DIR/kuken"
chmod +x "$INSTALL_DIR/kuken"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "Add the following to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo ""
echo "Küken CLI installed successfully!"
echo "Location: $INSTALL_DIR/kuken"
echo ""
echo "Run 'kuken --help' to get started"