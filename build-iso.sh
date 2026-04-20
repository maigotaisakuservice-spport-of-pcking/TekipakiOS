#!/bin/bash
# Tekipaki OS ISO Build Script (Local / CI)

set -e

echo "Starting Tekipaki OS v1.6 ISO Build..."

# 1. Build Rust binaries (Skip if in CI, handled by host - but wait, we need to build for Arch)
# We will build Rust binaries inside the Arch container in step 8

# 2. Build Custom Kernel (linux-tekipaki)
echo "Building Custom Kernel: linux-tekipaki (based on Zen)..."
KERNEL_DIR="/tmp/linux-tekipaki"
mkdir -p "$KERNEL_DIR"
git clone --depth 1 https://github.com/zen-kernel/zen-kernel.git "$KERNEL_DIR"
cd "$KERNEL_DIR"
# Optimization: use localmodconfig or a predefined tekipaki_config if available
# For now, use the default zen config with some 'tekipaki' branding
make x86_64_defconfig
sed -i 's/CONFIG_LOCALVERSION=""/CONFIG_LOCALVERSION="-tekipaki"/' .config
# Disable LOCALVERSION_AUTO to keep version string clean (no commit hashes)
sed -i 's/CONFIG_LOCALVERSION_AUTO=y/# CONFIG_LOCALVERSION_AUTO is not set/' .config
sed -i 's/CONFIG_HZ_1000=y/# CONFIG_HZ_1000 is not set\nCONFIG_HZ_1000=y/' .config # Ensure high responsiveness
make -j$(nproc) bzImage modules
cd -

# 3. Prepare ISO profile
PROFILE_DIR="tekipakios/iso-profile"
mkdir -p "$PROFILE_DIR/airootfs/usr/local/bin"
mkdir -p "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants"
mkdir -p "$PROFILE_DIR/airootfs/boot"

# 4. Inject Kernel and Modules
echo "Injecting kernel and modules..."
cp "$KERNEL_DIR/arch/x86/boot/bzImage" "$PROFILE_DIR/airootfs/boot/vmlinuz-linux-tekipaki"

# Install kernel and modules to the container root first so mkinitcpio can find them easily
echo "Installing kernel and modules to container root for mkinitcpio..."
KVER_INTERNAL=$(make -C "$KERNEL_DIR" -s kernelrelease)
cp "$KERNEL_DIR/arch/x86/boot/bzImage" "/boot/vmlinuz-$KVER_INTERNAL"
make -C "$KERNEL_DIR" INSTALL_MOD_PATH=/ modules_install

# Also install to airootfs for the final ISO
echo "Installing modules to airootfs..."
mkdir -p "$PROFILE_DIR/airootfs/usr/lib/modules"
make -C "$KERNEL_DIR" INSTALL_MOD_PATH="$(pwd)/$PROFILE_DIR/airootfs" modules_install

# Generate initramfs for the custom kernel
echo "Generating initramfs for linux-tekipaki..."
KVER=$(make -C "$KERNEL_DIR" -s kernelrelease)

# Ensure depmod is run for the new version
echo "Generating module dependencies..."
# 1. For the build container environment
depmod -a "$KVER"
# 2. For the ISO's root filesystem
depmod -a -b "$(pwd)/$PROFILE_DIR/airootfs" "$KVER"

# Failure here is critical, so we must not ignore it.
# We run mkinitcpio. It will search in /lib/modules (container) by default.
mkinitcpio -k "$KVER" -c "$PROFILE_DIR/mkinitcpio.conf" -g "$PROFILE_DIR/airootfs/boot/initramfs-linux-tekipaki.img"

# Update bootloader entries
echo "Updating bootloader entries..."
BOOT_ENTRY="$PROFILE_DIR/efiboot/loader/entries/tekipaki.conf"
mkdir -p "$(dirname "$BOOT_ENTRY")"
cat <<EOF > "$BOOT_ENTRY"
title   Tekipaki OS (linux-tekipaki)
linux   /boot/vmlinuz-linux-tekipaki
initrd  /boot/initramfs-linux-tekipaki.img
options archisobasedir=arch archisolabel=TEKIPAKI_$(date +%Y%m)
EOF

# Add F11 Recovery Entry
RECOVERY_ENTRY="$PROFILE_DIR/efiboot/loader/entries/tekipaki-recovery.conf"
cat <<EOF > "$RECOVERY_ENTRY"
title   Tekipaki Recovery Environment (TRE)
linux   /boot/vmlinuz-linux-tekipaki
initrd  /boot/initramfs-linux-tekipaki.img
options archisobasedir=arch archisolabel=TEKIPAKI_$(date +%Y%m) tekipaki_recovery=1
EOF

# Note: F11 mapping usually requires GRUB or systemd-boot with specific patches or config.
# For systemd-boot, we set the recovery as a secondary entry.

# Remove symlinks to build/source in airootfs to save space
if [ -d "$PROFILE_DIR/airootfs/usr/lib/modules" ]; then
    find "$PROFILE_DIR/airootfs/usr/lib/modules" -type l -delete
fi

# 5. Build and Inject Rust Binaries (Inside Arch environment)
echo "Building Tekipaki AppSet and Guard for Arch Linux..."
cargo build --release
echo "Injecting binaries into airootfs..."
mkdir -p "$PROFILE_DIR/airootfs/usr/local/bin"
cp target/release/tekipaki-* "$PROFILE_DIR/airootfs/usr/local/bin/"

# 5b. Secure Component Signing
if [ -n "$TEKIPAKI_PRIVATE_KEY" ]; then
    echo "Signing core components with TEKIPAKI_PRIVATE_KEY..."
    # Convert Base64 private key to DER for openssl
    echo "$TEKIPAKI_PRIVATE_KEY" | base64 -d > /tmp/tekipaki.key

    # Sign all custom binaries
    mkdir -p "$PROFILE_DIR/airootfs/etc/tekipaki/signatures"
    for bin in target/release/tekipaki-*; do
        bin_name=$(basename "$bin")
        openssl pkeyutl -sign -inkey /tmp/tekipaki.key -rawin -in "$bin" -out "$PROFILE_DIR/airootfs/etc/tekipaki/signatures/$bin_name.sig"
    done

    # Extract and inject public key for verification
    openssl pkey -in /tmp/tekipaki.key -pubout -outform DER -out "$PROFILE_DIR/airootfs/etc/tekipaki/public.key"
    rm /tmp/tekipaki.key
    echo "Signing complete. Public key injected."
else
    echo "Warning: TEKIPAKI_PRIVATE_KEY not set. Skipping component signing."
fi

# 6. Inject Systemd Services
cp tekipakios/config/systemd/*.service "$PROFILE_DIR/airootfs/etc/systemd/system/"
cp tekipakios/config/systemd/*.timer "$PROFILE_DIR/airootfs/etc/systemd/system/"
ln -sf /etc/systemd/system/tekipaki-cdrive.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/tekipaki-cdrive.service"
ln -sf /etc/systemd/system/tekipaki-guardd.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/tekipaki-guardd.service"
ln -sf /etc/systemd/system/tekipaki-init.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/tekipaki-init.service"
ln -sf /etc/systemd/system/tekipaki-update-check.timer "$PROFILE_DIR/airootfs/etc/systemd/system/timers.target.wants/tekipaki-update-check.timer"

# 7. Build AUR Packages
echo "Building AUR Packages..."
# Create a dedicated builder user (makepkg cannot run as root)
useradd -m builder || true
# Allow builder to use pacman without password (required for makepkg -s)
echo "builder ALL=(ALL) NOPASSWD: /usr/bin/pacman" > /etc/sudoers.d/builder-pacman

build_aur_pkg() {
    local pkg_name=$1
    local work_dir="/tmp/aur-$pkg_name"
    echo "Building $pkg_name..."
    mkdir -p "$work_dir"
    git clone https://aur.archlinux.org/$pkg_name.git "$work_dir"
    chown -R builder "$work_dir"

    # Import GPG keys if the package requires them
    if [ "$pkg_name" == "proton-ge-custom-bin" ]; then
        runuser -l builder -c "gpg --recv-keys 161D67634F754D22" || true
    fi

    # Run makepkg as builder, allowing it to use sudo pacman for deps
    # We use runuser to avoid using 'sudo' command directly in the script flow where possible
    runuser -l builder -c "cd $work_dir && makepkg -sc --noconfirm --needed"

    mkdir -p "$PROFILE_DIR/repo"
    cp "$work_dir"/*.pkg.tar.zst "$PROFILE_DIR/repo/"
    # Update local repo immediately to satisfy future AUR dependencies
    repo-add "$PROFILE_DIR/repo/tekipaki.db.tar.gz" "$PROFILE_DIR/repo/"*.pkg.tar.zst
}

# Ensure local repo is registered in pacman.conf for dependencies
mkdir -p "$(pwd)/$PROFILE_DIR/repo"
# Create an empty db if it doesn't exist to prevent pacman errors
if [ ! -f "$PROFILE_DIR/repo/tekipaki.db.tar.gz" ]; then
    tar czf "$PROFILE_DIR/repo/tekipaki.db.tar.gz" -T /dev/null
fi

if ! grep -q "\[tekipaki\]" /etc/pacman.conf; then
    cat <<EOF >> /etc/pacman.conf

[tekipaki]
SigLevel = Optional TrustAll
Server = file://$(pwd)/$PROFILE_DIR/repo
EOF
fi

# Build in order to satisfy dependencies
build_aur_pkg "archlinux-appstream-data-pamac"
build_aur_pkg "libpamac-aur"
build_aur_pkg "pamac-aur"
build_aur_pkg "proton-ge-custom-bin"

# Local repo is already updated by build_aur_pkg

if ! grep -q "\[tekipaki\]" "$PROFILE_DIR/pacman.conf"; then
    cat <<EOF >> "$PROFILE_DIR/pacman.conf"
[tekipaki]
SigLevel = Optional TrustAll
Server = file://$(pwd)/$PROFILE_DIR/repo
EOF
fi

# 8. Build ISO (Requires archiso)
echo "Starting mkarchiso..."
mkarchiso -v -w /tmp/archiso-tmp -o out "$PROFILE_DIR"

echo "Build complete."
