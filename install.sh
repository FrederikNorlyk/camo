#!/bin/bash

set -e

# Colors for output
GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo "Installing Camo..."

TEMP_DIR=$(mktemp -d)
trap 'rm -rf $TEMP_DIR' EXIT

# Download
echo "Downloading Camo..."
curl -sSL https://github.com/FrederikNorlyk/camo/releases/latest/download/camo -o "$TEMP_DIR/camo"
chmod +x "$TEMP_DIR/camo"
sudo mv "$TEMP_DIR/camo" /usr/local/bin/camo

echo -e "${GREEN}Installation complete!${NC}"
