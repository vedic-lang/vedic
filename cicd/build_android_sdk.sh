#!/bin/bash
set -euo pipefail

echo "::group::Setting up Android SDK and NDK"

# Set the ${ANDROID_HOME} variable
export ANDROID_HOME=${HOME}/android

# Clean up existing installation
rm -rf ${ANDROID_HOME}
mkdir -p ${ANDROID_HOME}

# Download and extract the command-line tools
cd ${HOME}
wget -q https://dl.google.com/android/repository/commandlinetools-linux-8512546_latest.zip \
    -O commandlinetools-linux-8512546_latest.zip

# Verify checksum
echo "2ccbda4302db862a28ada25aa7425d99dce9462046003c1714b059b5c47970d8  commandlinetools-linux-8512546_latest.zip" | sha256sum -c -

# Extract to proper location (cmdline-tools/latest)
mkdir -p ${ANDROID_HOME}/cmdline-tools
unzip -q commandlinetools-linux-8512546_latest.zip -d ${ANDROID_HOME}/cmdline-tools
mv ${ANDROID_HOME}/cmdline-tools/cmdline-tools ${ANDROID_HOME}/cmdline-tools/latest
rm commandlinetools-linux-8512546_latest.zip

# Add to PATH
export PATH=${PATH}:${ANDROID_HOME}/cmdline-tools/latest/bin:${ANDROID_HOME}/platform-tools

# Accept licenses and install required components
yes | sdkmanager --licenses

echo "Installing Android SDK components..."
sdkmanager --install \
    "build-tools;30.0.3" \
    "ndk;25.1.8937393" \
    "platforms;android-33" \
    "platform-tools"

# Set NDK environment variables
export NDK_HOME=${ANDROID_HOME}/ndk/25.1.8937393
export PATH=${PATH}:${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin

# Verify installation
echo "Android SDK installed at: ${ANDROID_HOME}"
echo "Android NDK installed at: ${NDK_HOME}"
ls -la ${NDK_HOME}/toolchains/llvm/prebuilt/linux-x86_64/bin/ | head -20

echo "::endgroup::"
