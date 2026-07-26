#!/usr/bin/env bash
set -e

if ! command -v cargo &>/dev/null; then
    echo "Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

cargo build --release

cp target/release/CFGBeast .
chmod +x CFGBeast
sha256sum CFGBeast > CFGBeast.sha256.txt

echo "Build complete."
cat CFGBeast.sha256.txt
