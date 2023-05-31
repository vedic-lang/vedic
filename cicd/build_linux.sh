#!/bin/bash

# Create release folder
mkdir -p release_linux

# Install Dependencies
sudo apt update
sudo apt upgrade -y
sudo apt install -y gcc-aarch64-linux-gnu
sudo apt install -y binutils-aarch64-linux-gnu
sudo apt install -y gcc-arm-linux-gnueabihf
sudo apt install -y binutils-arm-linux-gnueabihf
sudo apt install -y musl-tools
sudo apt install -y gcc-aarch64-linux-gnu
sudo apt install -y binutils-aarch64-linux-gnu
sudo apt install -y gcc-i686-linux-gnu

rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu
rustup target add armv7-unknown-linux-gnueabihf
rustup target add i686-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-musl
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add i686-linux-android
rustup target add x86_64-linux-android

cargo install cargo-deb
cargo install cross --git https://github.com/cross-rs/cross

# Build the Debian package for (default : GNU x86_64)
cargo deb -p vedic
cp ./target/debian/vedic_*.deb ./release_linux/vedic-linux-x86_64.deb

# Build the binary
build_binary() {
   echo -e "\nBuilding $2"
   rm -rf target
   if [ $1 == "cross" ]; then
      cross build --package vedic --release --target $2
   else
      cargo build --package vedic --release --target $2
   fi
   if [ -f "./target/$2/release/vedic" ]; then
      tar -czvf ./release_linux/$3 -C ./target/$2/release vedic
      echo -e "Build completed ./release_linux/$3 Genrated\n"
   else
      echo -e "Build failed $3\n"
   fi
}

build_binary default x86_64-unknown-linux-gnu vedic-linux-gnu-x86_64.tar.xz
build_binary default aarch64-unknown-linux-gnu vedic-linux-gnu-aarch64.tar.xz
build_binary default armv7-unknown-linux-gnueabihf vedic-linux-gnueabihf-armv7.tar.xz
build_binary default i686-unknown-linux-gnu vedic-linux-gnu-i686.tar.xz
build_binary default x86_64-unknown-linux-musl vedic-linux-musl-x86_64.tar.xz
build_binary cross aarch64-unknown-linux-musl vedic-linux-musl-aarch64.tar.xz
build_binary cross aarch64-linux-android vedic-android-aarch64.tar.xz
build_binary cross armv7-linux-androideabi vedic-android-armv7.tar.xz
build_binary cross i686-linux-android vedic-android-i686.tar.xz
build_binary cross x86_64-linux-android vedic-android-x86_64.tar.xz
