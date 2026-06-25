#!/bin/sh
set -e

# APCN Installer Script
# Can be run via: curl -sS <script-url> | sh
# Or with rug backend: BACKEND=rug curl -sS <script-url> | sh

REPO="BreezeWhite/apcn-rs"

# Detect OS and architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
  linux)
    PLATFORM="linux"
    ;;
  darwin)
    PLATFORM="macos"
    ;;
  *)
    echo "Error: Unsupported operating system '$OS'." >&2
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)
    ARCH_NAME="x86_64"
    ;;
  arm64|aarch64)
    ARCH_NAME="aarch64"
    ;;
  *)
    # Default to x86_64 as a fallback or print error
    ARCH_NAME="x86_64"
    ;;
esac

# Default backend to dashu for compatibility (zero runtime C dependencies),
# but allow users to specify BACKEND=rug for high-performance builds.
BACKEND="${BACKEND:-dashu}"

if [ "$PLATFORM" = "macos" ]; then
  # Mac builds in CI are targeted for Apple Silicon (aarch64)
  ARCH_NAME="aarch64"
fi

# Fetch the latest release (including pre-releases)
echo "Fetching latest release version..."
RELEASE_JSON=$(curl -sS "https://api.github.com/repos/$REPO/releases")
TAG=$(echo "$RELEASE_JSON" | grep '"tag_name":' | head -n1 | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')

if [ -z "$TAG" ]; then
  echo "Error: Could not retrieve release tags. Please try again." >&2
  exit 1
fi

ARCHIVE_NAME="apcn-${BACKEND}-${PLATFORM}-${ARCH_NAME}.tar.gz"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/${TAG}/${ARCHIVE_NAME}"

# Determine writeable install directory
if [ -w "/usr/local/bin" ]; then
  INSTALL_DIR="/usr/local/bin"
  USE_SUDO="false"
elif [ -d "$HOME/.local/bin" ] && [ -w "$HOME/.local/bin" ]; then
  INSTALL_DIR="$HOME/.local/bin"
  USE_SUDO="false"
elif [ -d "$HOME/bin" ] && [ -w "$HOME/bin" ]; then
  INSTALL_DIR="$HOME/bin"
  USE_SUDO="false"
else
  # Use sudo if /usr/local/bin isn't directly writeable but sudo is available
  if command -v sudo >/dev/null 2>&1; then
    INSTALL_DIR="/usr/local/bin"
    USE_SUDO="true"
  else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
    USE_SUDO="false"
  fi
fi

# Create a temporary directory for download
TMP_DIR=$(mktemp -d)
clean_up() {
  rm -rf "$TMP_DIR"
}
trap clean_up EXIT

echo "Downloading apcn $TAG ($BACKEND backend for $PLATFORM-$ARCH_NAME)..."
curl -L -f -sS "$DOWNLOAD_URL" -o "$TMP_DIR/archive.tar.gz"

if [ ! -s "$TMP_DIR/archive.tar.gz" ]; then
  echo "Error: Failed to download release asset from $DOWNLOAD_URL" >&2
  exit 1
fi

echo "Extracting binary..."
tar -xzf "$TMP_DIR/archive.tar.gz" -C "$TMP_DIR"

if [ ! -f "$TMP_DIR/apcn" ]; then
  echo "Error: Binary file 'apcn' not found inside the downloaded archive." >&2
  exit 1
fi

echo "Installing binary to $INSTALL_DIR/apcn..."
if [ "$USE_SUDO" = "true" ]; then
  sudo mv "$TMP_DIR/apcn" "$INSTALL_DIR/apcn"
  sudo chmod +x "$INSTALL_DIR/apcn"
else
  mv "$TMP_DIR/apcn" "$INSTALL_DIR/apcn"
  chmod +x "$INSTALL_DIR/apcn"
fi

echo "Successfully installed apcn to $INSTALL_DIR/apcn!"

# Verify if install path is in PATH variable
case ":$PATH:" in
  *:"$INSTALL_DIR":*)
    echo "You can now run: apcn --help"
    ;;
  *)
    echo "\nWarning: $INSTALL_DIR is not in your current PATH environment variable."
    echo "To be able to run 'apcn', please add it to your shell configuration profile (e.g. ~/.bashrc or ~/.zshrc):"
    echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
    ;;
esac
