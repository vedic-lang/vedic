#!/bin/bash
set -euo pipefail

echo "::group::Set-up Android Build Environment"

# Source the Android SDK setup script
source cicd/build_android_sdk.sh

# Create release folder
mkdir -p release_android

# Create .cargo/config.toml for Android NDK linkers
mkdir -p .cargo
cat > .cargo/config.toml << 'EOF'
[target.aarch64-linux-android]
linker = "aarch64-linux-android30-clang"

[target.armv7-linux-androideabi]
linker = "armv7a-linux-androideabi30-clang"

[target.i686-linux-android]
linker = "i686-linux-android30-clang"

[target.x86_64-linux-android]
linker = "x86_64-linux-android30-clang"
EOF

echo "::endgroup::"

# Build Android binary
build_android_binary() {
   local target=$1
   local output_name=$2
   
   echo "::group::Building Android $target"
   
   # Clean previous build
   rm -rf target/$target
   
   # Build
   cargo build --package vedic --release --target $target
   
   if [ -f "./target/$target/release/vedic" ]; then
      # Create tarball
      tar -cJf ./release_android/$output_name -C ./target/$target/release vedic
      
      # Get file size
      size=$(du -h "./release_android/$output_name" | cut -f1)
      echo "✓ Build completed: ./release_android/$output_name ($size)"
   else
      echo "✗ Build failed: $output_name"
      exit 1
   fi
   
   echo "::endgroup::"
}

# Build for all Android architectures
build_android_binary aarch64-linux-android vedic-android-aarch64.tar.xz
build_android_binary armv7-linux-androideabi vedic-android-armv7.tar.xz
build_android_binary i686-linux-android vedic-android-i686.tar.xz
build_android_binary x86_64-linux-android vedic-android-x86_64.tar.xz

echo "::group::Build Summary"
echo "Android builds completed successfully:"
ls -lh release_android/
echo "::endgroup::"
