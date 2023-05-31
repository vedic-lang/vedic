# download and install rust (Not required if rust is already installed)
# Invoke-WebRequest -Uri 'https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe' -OutFile rustup-init.exe
# .\rustup-init.exe -y --default-toolchain stable-x86_64-pc-windows-msvc --default-host x86_64-pc-windows-msvc

# add rust to path
# $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"

# download and setup wix
# Invoke-WebRequest -Uri 'https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip' -OutFile wix.zip
# Expand-Archive -LiteralPath wix.zip -DestinationPath wix

# create release folder
New-Item -ItemType Directory -Force -Path ./release_windows

# install cargo-wix
cargo install cargo-wix --version 0.3.3

# build the binary
cargo build --package vedic --release

# build the installer
cargo wix --package vedic --no-build --nocapture --output ./release_windows/vedic-win-x86_64-installer.msi

# zip the binary
Compress-Archive -Path ./target/release/vedic.exe -DestinationPath ./release_windows/vedic-win-x86_64.zip

ls ./release_windows
