#!/bin/bash
set -euo pipefail

echo "::group::Set-up Cross for Android"

# Install cross
cargo install cross --git https://github.com/cross-rs/cross

# Create release folder
mkdir -p release_android

echo "::endgroup::"

export CC_aarch64_linux_android=aarch64-linux-android30-clang
export CC_armv7_linux_androideabi=armv7a-linux-androideabi30-clang

# Build with cross (handles all Android SDK/NDK setup automatically)
build_android() {
   local target=$1
   local output=$2
   
   echo "::group::Building $target"
   cross build --package vedic --release --target $target
   
   if [ -f "./target/$target/release/vedic" ]; then
      tar -cJf ./release_android/$output -C ./target/$target/release vedic
      echo "✓ Build completed: $output"
   else
      echo "✗ Build failed: $output"
      exit 1
   fi
   echo "::endgroup::"
}

build_android aarch64-linux-android vedic-android-aarch64.tar.xz
build_android armv7-linux-androideabi vedic-android-armv7.tar.xz
build_android i686-linux-android vedic-android-i686.tar.xz
build_android x86_64-linux-android vedic-android-x86_64.tar.xz

echo "::group::Build Summary"
ls -lh release_android/
echo "Android builds completed successfully:"
echo "::endgroup::"
