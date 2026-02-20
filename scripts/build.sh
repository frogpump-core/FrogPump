#!/bin/bash
set -euo pipefail

echo "Building FrogPump CLI (release)..."
cargo build --release

echo "Build complete: target/release/frogpump"
ls -lh target/release/frogpump
