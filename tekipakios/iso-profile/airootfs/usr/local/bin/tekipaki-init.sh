#!/bin/bash
# Tekipaki OS First Boot Initialization

# Enable Snap support
systemctl enable --now snapd.socket
ln -sf /var/lib/snapd/snap /snap

# Enable Flatpak Flathub repository
flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo

# Other init tasks...
echo "Tekipaki OS Initialization Complete."
