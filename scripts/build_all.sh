#!/bin/bash
set -e # Exit on error


# MSVC TARGETS (Using cargo-xwin)
echo "Building Windows MSVC targets..."
cargo xwin build --target x86_64-pc-windows-msvc --release --xwin-arch x86_64
cargo xwin build --target i686-pc-windows-msvc --release --xwin-arch x86
cargo xwin build --target aarch64-pc-windows-msvc --release --xwin-arch aarch64

# LINUX TARGETS (Standard or Cross)
# Note: For musl/arm, you might need 'cross' if you don't want to install local toolchains
echo "Building Linux targets..."
cargo build --target x86_64-unknown-linux-gnu --release
cross build --target x86_64-unknown-linux-musl --release
cross build --target aarch64-unknown-linux-gnu --release
cross build --target arm-unknown-linux-gnueabi --release

echo "All builds completed successfully!"