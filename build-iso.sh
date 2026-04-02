#!/bin/bash
# Tekipaki OS ISO Build Script (Local / CI)

set -e

echo "Starting Tekipaki OS v1.6 ISO Build..."

# 1. Build Rust binaries
echo "Building Tekipaki AppSet and Guard..."
cargo build --release

# 2. Prepare ISO profile
PROFILE_DIR="tekipakios/iso-profile"
mkdir -p "$PROFILE_DIR/airootfs/usr/local/bin"
mkdir -p "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants"

# 3. Inject Binaries
echo "Injecting binaries into airootfs..."
cp target/release/tekipaki-* "$PROFILE_DIR/airootfs/usr/local/bin/"

# 4. Inject Systemd Services
cp tekipakios/config/systemd/*.service "$PROFILE_DIR/airootfs/etc/systemd/system/"
ln -sf /etc/systemd/system/tekipaki-cdrive.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/tekipaki-cdrive.service"

# 5. Build ISO (Requires archiso and root/docker)
if [ "$CI" = "true" ]; then
    echo "Running in CI, letting workflow handle mkarchiso via docker."
else
    echo "Running locally. Attempting mkarchiso..."
    if command -v mkarchiso &> /dev/null; then
        sudo mkarchiso -v -w /tmp/archiso-tmp -o out "$PROFILE_DIR"
    else
        echo "Error: mkarchiso not found. Please install 'archiso' package."
        exit 1
    fi
fi

echo "Build complete."
