#!/bin/bash

# Tekipaki OS archiso profile definition

iso_name="tekipaki-os"
iso_label="TEKIPAKI_$(date +%Y%m)"
iso_publisher="Tekipaki Project <https://github.com/tekipaki-os>"
iso_application="Tekipaki OS Live/Installation Media"
iso_version="1.6"
install_dir="arch"
buildmodes=('iso')
bootmodes=('bios.syslinux.mbr' 'bios.syslinux.eltorito' 'uefi-x64.systemd-boot.esp' 'uefi-x64.systemd-boot.eltorito')
arch="x86_64"
pacman_conf="pacman.conf"
airootfs_image_type="squashfs"
airootfs_image_tool_options=('-comp' 'xz' '-Xbcj' 'x86')
file_permissions=(
  ["/etc/shadow"]="0:0:400"
  ["/root"]="0:0:700"
  ["/usr/local/bin/tekipaki-setup"]="0:0:755"
  ["/usr/local/bin/tekipaki-guardd"]="0:0:755"
  ["/usr/local/bin/tekipaki-cli"]="0:0:755"
  ["/usr/local/bin/tekipaki-keygen"]="0:0:755"
  ["/usr/local/bin/tekipaki-settings"]="0:0:755"
  ["/usr/local/bin/tekipaki-note"]="0:0:755"
  ["/usr/local/bin/tekipaki-paint"]="0:0:755"
  ["/usr/local/bin/tekipaki-calc"]="0:0:755"
  ["/usr/local/bin/tekipaki-taskmgr"]="0:0:755"
  ["/usr/local/bin/tekipaki-tre"]="0:0:755"
  ["/usr/local/bin/tekipaki-init.sh"]="0:0:755"
  ["/usr/local/bin/tekipaki-reboot-countdown.sh"]="0:0:755"
  ["/usr/local/bin/tekipaki-update-check.sh"]="0:0:755"
  ["/usr/lib/initcpio/install/archiso"]="0:0:755"
  ["/usr/lib/initcpio/hooks/archiso"]="0:0:755"
)
