#!/bin/bash

if [ "$(uname)" != "Darwin" ]; then
  echo "This installer is only available for macOS."
  exit 1
fi

BINARY_URL="https://raw.githubusercontent.com/SouchonTheo/git-flip/main/release/git-flip"

TARGET_DIR="/usr/local/bin"
TARGET_BIN="${TARGET_DIR}/git-flip"

echo "Downloading git-flip for macOS..."
curl -sSfL "$BINARY_URL" -o git-flip || { echo "Error downloading the binary."; exit 1; }

chmod +x git-flip

if [ "$(id -u)" -ne 0 ]; then
  echo "Moving the binary to ${TARGET_DIR} (sudo required)..."
  sudo mv git-flip "$TARGET_BIN" || { echo "Error moving the binary."; exit 1; }
else
  mv git-flip "$TARGET_BIN" || { echo "Error moving the binary."; exit 1; }
fi

echo "Installation complete. You can now run 'git-flip' from your terminal."