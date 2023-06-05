#!/bin/bash

set -e

echo "Building taskman..."
echo "Running ($ cargo build --release)..."
cargo build --release

if [ $? -ne 0 ]; then
  echo "Failed to build taskman."
  exit 1
fi

if command -v upx &> /dev/null; then
  echo "Compressing taskman's Executable..."
  echo "Running ($ upx --best --lzma target/release/taskman)..."
  upx --best --lzma target/release/taskman
fi

echo "Copying taskman to the /usr/local/bin/ directory..."
echo "Running (# cp ./target/release/taskman /usr/local/bin/taskman)..."
sudo cp ./target/release/taskman /usr/local/bin/taskman

if [ $? -ne 0 ]; then
  echo "Failed to copy taskman to /usr/local/bin/."
  exit 1
fi

echo "Alhamdulillah! Exiting... "
