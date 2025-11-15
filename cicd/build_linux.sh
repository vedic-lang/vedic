#!/bin/bash
set -euo pipefail

echo "::group::Set-up Build Dependencies"

# Create release folder
mkdir -p release_linux

# Install Linux cross-compilation dependencies
sudo apt update
sudo apt upgrade -y
sudo apt install -y \
    gcc-aarch64-linux-gnu \
    binutils-aarch64-linux-gnu \
    gcc-arm-linux-gnueabihf \
    binutils-arm-linux-gnueabihf \
    gcc-i686-linux-gnu \
    binutils-i686-linux-gnu \
    musl-tools

# Add Rust targets for Linux
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
rustup target add i686-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl

# Add Rust targets for Android
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

# Install cargo tools
cargo install cargo-deb
cargo install cross --git https://github.com/cross-rs/cross

echo "::endgroup::"

echo "::group::Building Debian package"
# Build the Debian package for (default : GNU x86_64)
cargo deb -p vedic
cp ./target/debian/vedic_*.deb ./release_linux/vedic-linux-x86_64.deb
echo "::endgroup::"

# Build the binary
build_binary() {
   echo "::group::Building $2 ($3)"
   rm -rf target/$2
   
   if [ "$1" == "cross" ]; then
      cross build --package vedic --release --target $2
   else
      cargo build --package vedic --release --target $2
   fi
   
   if [ -f "./target/$2/release/vedic" ]; then
      tar -cJf ./release_linux/$3 -C ./target/$2/release vedic
      echo "✓ Build completed: ./release_linux/$3"
   else
      echo "✗ Build failed: $3"
      exit 1
   fi
   echo "::endgroup::"
}

# Linux builds
build_binary default x86_64-unknown-linux-gnu vedic-linux-gnu-x86_64.tar.xz
build_binary default aarch64-unknown-linux-gnu vedic-linux-gnu-aarch64.tar.xz
build_binary default armv7-unknown-linux-gnueabihf vedic-linux-gnueabihf-armv7.tar.xz
build_binary default i686-unknown-linux-gnu vedic-linux-gnu-i686.tar.xz
build_binary default x86_64-unknown-linux-musl vedic-linux-musl-x86_64.tar.xz
build_binary cross aarch64-unknown-linux-musl vedic-linux-musl-aarch64.tar.xz

echo "Linux builds completed successfully"
