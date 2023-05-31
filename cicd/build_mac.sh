#!/bin/bash

# Create release folder
mkdir -p release_mac

# Build binary (x86_64)
cargo build --package vedic --release --target x86_64-apple-darwin
tar -czvf ./release_mac/vedic-darwin-x86_64.tar.gz -C ./target/x86_64-apple-darwin/release vedic

# Build binary (aarch64)
rustup target add --toolchain stable aarch64-apple-darwin
cargo build --package vedic --release --target aarch64-apple-darwin
tar -czvf ./release_mac/vedic-darwin-aarch64.tar.gz -C ./target/aarch64-apple-darwin/release vedic

# # Unify binaries
# lipo -create -output \
#     ./vedic \
#     ./target/x86_64-apple-darwin/release/vedic \
#     ./target/aarch64-apple-darwin/release/vedic
# chmod +x ./vedic
# tar -czvf ./release_mac/vedic-mac.tar.gz ./vedic
# Build the installer
# cargo bundle --package vedic --release
# ls ./target/bundle
# cp ./target/bundle/vedic*.app ./release_mac/vedic-v$version-mac-x64.app
