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
  mingw*|msys*|cygwin*)
    PLATFORM="windows"
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

# To download dashu backend version, set env var BACKEND=dashu
BACKEND="${BACKEND:-rug}"

if [ "$PLATFORM" = "macos" ]; then
  # Mac builds in CI are targeted for Apple Silicon (aarch64)
  ARCH_NAME="aarch64"
fi

if [ "$PLATFORM" = "windows" ]; then
  BINARY_NAME="apcn.exe"
  ARCHIVE_EXT="zip"
else
  BINARY_NAME="apcn"
  ARCHIVE_EXT="tar.gz"
fi

# Fetch the latest release (including pre-releases)
echo "Fetching latest release version..."
RELEASE_JSON=$(curl -sS "https://api.github.com/repos/$REPO/releases")
TAG=$(echo "$RELEASE_JSON" | grep '"tag_name":' | head -n1 | sed -E 's/.*"tag_name": "([^"]+)".*/\1/')

if [ -z "$TAG" ]; then
  echo "Error: Could not retrieve release tags. Please try again." >&2
  exit 1
fi

ARCHIVE_NAME="apcn-${BACKEND}-${PLATFORM}-${ARCH_NAME}.${ARCHIVE_EXT}"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/${TAG}/${ARCHIVE_NAME}"

# Determine writeable install directory
if [ "$PLATFORM" = "windows" ]; then
  INSTALL_DIR="$HOME/.local/bin"
  mkdir -p "$INSTALL_DIR"
  USE_SUDO="false"
elif [ -w "/usr/local/bin" ]; then
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
curl -L -f -sS "$DOWNLOAD_URL" -o "$TMP_DIR/archive.${ARCHIVE_EXT}"

if [ ! -s "$TMP_DIR/archive.${ARCHIVE_EXT}" ]; then
  echo "Error: Failed to download release asset from $DOWNLOAD_URL" >&2
  exit 1
fi

echo "Extracting binary..."
if [ "$PLATFORM" = "windows" ]; then
  if command -v unzip >/dev/null 2>&1; then
    unzip -q "$TMP_DIR/archive.zip" -d "$TMP_DIR"
  elif command -v tar >/dev/null 2>&1; then
    tar -xf "$TMP_DIR/archive.zip" -C "$TMP_DIR"
  else
    powershell -Command "Expand-Archive -Path '$TMP_DIR/archive.zip' -DestinationPath '$TMP_DIR' -Force"
  fi
else
  tar -xzf "$TMP_DIR/archive.tar.gz" -C "$TMP_DIR"
fi

if [ ! -f "$TMP_DIR/$BINARY_NAME" ]; then
  echo "Error: Binary file '$BINARY_NAME' not found inside the downloaded archive." >&2
  exit 1
fi

echo "Installing binary to $INSTALL_DIR/$BINARY_NAME..."
if [ "$USE_SUDO" = "true" ]; then
  sudo mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
  sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"
else
  mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
  chmod +x "$INSTALL_DIR/$BINARY_NAME"
fi

echo "Successfully installed apcn to $INSTALL_DIR/$BINARY_NAME!"

# Verify if install path is in PATH variable
PATH_OK="false"
case ":$PATH:" in
  *:"$INSTALL_DIR":*)
    PATH_OK="true"
    ;;
esac
case ":$PATH:" in
  *:"$INSTALL_DIR":*)
    PATH_OK="true"
    ;;
esac

if [ "$PATH_OK" = "true" ]; then
  echo "You can now run: apcn --help"
else
  echo "\nWarning: $INSTALL_DIR is not in your current PATH environment variable."
  echo "To be able to run 'apcn', please add it to your shell configuration profile (e.g. ~/.bashrc or ~/.zshrc):"
  echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi
