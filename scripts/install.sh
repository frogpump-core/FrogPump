#!/bin/bash
set -euo pipefail

echo "Installing FrogPump CLI..."
cargo install --path .

echo "FrogPump CLI installed successfully."
echo "Run 'frogpump --help' to get started."
