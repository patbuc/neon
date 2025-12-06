#!/bin/bash
# Install Neon extension for Zed in development mode

set -e

# Zed extensions directory
ZED_EXTENSIONS_DIR="$HOME/.config/zed/extensions"
NEON_EXTENSION_DIR="$ZED_EXTENSIONS_DIR/neon"

# Create extensions directory if it doesn't exist
mkdir -p "$ZED_EXTENSIONS_DIR"

# Remove existing installation if present
if [ -L "$NEON_EXTENSION_DIR" ] || [ -d "$NEON_EXTENSION_DIR" ]; then
  echo "Removing existing Neon extension..."
  rm -rf "$NEON_EXTENSION_DIR"
fi

# Get the absolute path to the extension
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
EXTENSION_SOURCE="$SCRIPT_DIR/neon"

# Create symlink for development
echo "Installing Neon extension in development mode..."
ln -s "$EXTENSION_SOURCE" "$NEON_EXTENSION_DIR"

echo "✓ Neon extension installed!"
echo ""
echo "Next steps:"
echo "1. Restart Zed or run 'Reload Window' command"
echo "2. Open a .neon or .n file"
echo "3. Verify syntax highlighting is working"
echo ""
echo "To uninstall: rm -rf $NEON_EXTENSION_DIR"
