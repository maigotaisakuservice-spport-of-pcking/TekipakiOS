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
make -C "$KERNEL_DIR" INSTALL_MOD_PATH="$(pwd)/$PROFILE_DIR/airootfs" modules_install

# Generate initramfs for the custom kernel
echo "Generating initramfs for linux-tekipaki..."
# We need to run mkinitcpio. Since we are in a container, we might need some trickery
# but usually archiso's mkinitcpio works.
# However, we need to point it to the modules we just installed in airootfs
KVER=$(make -C "$KERNEL_DIR" -s kernelrelease)
mkinitcpio -k "$KVER" -c "$PROFILE_DIR/mkinitcpio.conf" -g "$PROFILE_DIR/airootfs/boot/initramfs-linux-tekipaki.img" -d "$(pwd)/$PROFILE_DIR/airootfs" || \
echo "Warning: mkinitcpio failed in container, ensure dependencies are met."

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
find "$PROFILE_DIR/airootfs/usr/lib/modules" -type l -delete

# 5. Build and Inject Rust Binaries (Inside Arch environment)
echo "Building Tekipaki AppSet and Guard for Arch Linux..."
cargo build --release
echo "Injecting binaries into airootfs..."
mkdir -p "$PROFILE_DIR/airootfs/usr/local/bin"
cp target/release/tekipaki-* "$PROFILE_DIR/airootfs/usr/local/bin/"

# 5b. Inject License Key if present
if [ -n "$TEKIPAKI_PRIVATE_KEY" ]; then
    echo "Injecting license key..."
    mkdir -p "$PROFILE_DIR/airootfs/etc/tekipaki"
    echo "$TEKIPAKI_PRIVATE_KEY" > "$PROFILE_DIR/airootfs/etc/tekipaki/license.key"
fi

# 6. Inject Systemd Services
cp tekipakios/config/systemd/*.service "$PROFILE_DIR/airootfs/etc/systemd/system/"
ln -sf /etc/systemd/system/tekipaki-cdrive.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/tekipaki-cdrive.service"
ln -sf /etc/systemd/system/tekipaki-guardd.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/tekipaki-guardd.service"
ln -sf /etc/systemd/system/tekipaki-init.service "$PROFILE_DIR/airootfs/etc/systemd/system/multi-user.target.wants/tekipaki-init.service"

# 7. Build AUR Packages
echo "Building AUR Packages..."
# Prepare nobody user for sudo (required for makepkg -s)
echo "nobody ALL=(ALL) NOPASSWD: /usr/bin/pacman" > /etc/sudoers.d/nobody-pacman

build_aur_pkg() {
    local pkg_name=$1
    local work_dir="/tmp/aur-$pkg_name"
    echo "Building $pkg_name..."
    mkdir -p "$work_dir"
    git clone https://aur.archlinux.org/$pkg_name.git "$work_dir"
    chown -R nobody "$work_dir"
    # Run makepkg as nobody, allowing it to use sudo pacman for deps
    # We use --syncdeps to install dependencies from official repos.
    # Note: If AUR dependencies are needed, they should be built in order.
    sudo -u nobody bash -c "cd $work_dir && makepkg -sc --noconfirm"
    mkdir -p "$PROFILE_DIR/repo"
    cp "$work_dir"/*.pkg.tar.zst "$PROFILE_DIR/repo/"
    # Update local repo immediately to satisfy future AUR dependencies
    repo-add "$PROFILE_DIR/repo/tekipaki.db.tar.gz" "$PROFILE_DIR/repo/"*.pkg.tar.zst
}

# Ensure local repo is registered in pacman.conf for dependencies
mkdir -p "$PROFILE_DIR/repo"
touch "$PROFILE_DIR/repo/tekipaki.db.tar.gz"
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
