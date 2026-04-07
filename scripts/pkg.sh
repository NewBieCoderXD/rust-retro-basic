#!/bin/bash

# Configuration
PROJECT_NAME="rust-retro-basic"
VERSION="v1.0.0"
DIST_DIR="dist"

# Create a fresh dist folder
rm -rf $DIST_DIR
mkdir -p $DIST_DIR

echo "Archiving and collecting binaries into ./$DIST_DIR..."

# Function to package Windows (ZIP)
pkg_win() {
    local target=$1
    local out_name="${PROJECT_NAME}-${VERSION}-${target}.zip"
    # Zip the .exe from the target folder
    zip -j "$DIST_DIR/$out_name" "target/$target/release/${PROJECT_NAME}.exe"
}

# Function to package Unix (TAR.GZ)
pkg_unix() {
    local target=$1
    local out_name="${PROJECT_NAME}-${VERSION}-${target}.tar.gz"
    # Tar the binary (preserves executable permissions)
    tar -C "target/$target/release" -czf "$DIST_DIR/$out_name" "${PROJECT_NAME}"
}

# Windows MSVC (ZIP)
pkg_win "x86_64-pc-windows-msvc"
pkg_win "i686-pc-windows-msvc"
pkg_win "aarch64-pc-windows-msvc"

# Linux (TAR.GZ)
pkg_unix "x86_64-unknown-linux-gnu"
pkg_unix "x86_64-unknown-linux-musl"
pkg_unix "aarch64-unknown-linux-gnu"
pkg_unix "arm-unknown-linux-gnueabi"

# Create Checksums (Crucial for GitHub Releases)
cd $DIST_DIR
sha256sum * > SHA256SUMS
cd ..

echo "Done! Upload the contents of the '$DIST_DIR' folder to GitHub."