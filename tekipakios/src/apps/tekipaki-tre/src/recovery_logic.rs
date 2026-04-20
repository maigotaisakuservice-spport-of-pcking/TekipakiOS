use std::process::Command;
use std::fs;
use std::path::Path;

/// Find the Tekipaki root partition by looking for a partition with the label 'TEKIPAKI_ROOT'
fn find_root_partition() -> Option<String> {
    let output = Command::new("lsblk")
        .args(["-no", "PATH,LABEL"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == "TEKIPAKI_ROOT" {
            return Some(parts[0].to_string());
        }
    }
    None
}

/// Ensure the root partition is mounted to /mnt
fn ensure_mounted() -> Result<String, String> {
    if Path::new("/mnt/etc/fstab").exists() {
        return Ok("/mnt".to_string());
    }

    let dev = find_root_partition().ok_or("Tekipaki root partition not found / Tekipakiパーティションが見つかりません")?;
    let status = Command::new("mount").args([&dev, "/mnt"]).status();

    if status.map(|s| s.success()).unwrap_or(false) {
        Ok("/mnt".to_string())
    } else {
        Err(format!("Failed to mount {} to /mnt", dev))
    }
}

pub fn check_system_integrity() -> Result<(), String> {
    println!("TRE: Starting Self-Healing...");
    let mnt = ensure_mounted()?;

    // 1. File System Check
    let dev = find_root_partition().unwrap();
    let _ = Command::new("umount").arg(&mnt).status(); // Must be unmounted for fsck
    let fsck_status = Command::new("fsck").args(["-y", &dev]).status();
    let _ = Command::new("mount").args([&dev, &mnt]).status(); // Remount

    if !fsck_status.map(|s| s.success()).unwrap_or(false) {
        return Err("File system repair failed / ファイルシステムの修復に失敗しました".to_string());
    }

    // 2. Reinstall critical packages via pacman --sysroot
    let status = Command::new("pacman")
        .args(["--sysroot", &mnt, "-Syu", "--noconfirm", "base", "linux-tekipaki", "grub", "tekipaki-guardd"])
        .status();

    match status {
        Ok(s) if s.success() => Ok(()),
        _ => Err("Package repair failed / パッケージの修復に失敗しました".to_string()),
    }
}

pub fn cloud_reinstall() -> Result<(), String> {
    println!("TRE: Initiating Cloud Reinstall...");

    // In TRE, we assume network is already configured (via tekipaki-init or manual)
    // 1. Format the root partition (DANGEROUS)
    let dev = find_root_partition().ok_or("Root partition not found")?;
    let _ = Command::new("umount").arg("/mnt").status();
    let fmt_status = Command::new("mkfs.ext4").args(["-F", "-L", "TEKIPAKI_ROOT", &dev]).status();

    if !fmt_status.map(|s| s.success()).unwrap_or(false) {
        return Err("Format failed / フォーマットに失敗しました".to_string());
    }

    // 2. Mount and Pacstrap
    let _ = Command::new("mount").args([&dev, "/mnt"]).status();
    let status = Command::new("pacstrap")
        .args(["/mnt", "base", "linux-tekipaki", "plasma-desktop", "grub", "tekipaki-guardd"])
        .status();

    // 3. Generate fstab
    let fstab_output = Command::new("genfstab").args(["-U", "/mnt"]).output().map_err(|e| e.to_string())?;
    fs::write("/mnt/etc/fstab", fstab_output.stdout).map_err(|e| e.to_string())?;

    match status {
        Ok(s) if s.success() => Ok(()),
        _ => Err("Cloud reinstall failed / クラウド再インストールに失敗しました".to_string()),
    }
}

pub fn reset_this_pc() -> Result<(), String> {
    println!("TRE: Resetting PC (Wiping User Data)...");
    let mnt = ensure_mounted()?;

    // Wipe /home and /var/lib/AccountsService to reset users
    let _ = Command::new("rm").args(["-rf", &format!("{}/home/*", mnt)]).status();
    let _ = Command::new("rm").args(["-rf", &format!("{}/var/lib/AccountsService/users/*", mnt)]).status();

    // Also remove the license file to force re-activation
    let _ = Command::new("rm").arg(&format!("{}/etc/tekipaki/license.key", mnt)).status();

    Ok(())
}

pub fn open_command_prompt() {
    println!("TRE: Opening Command Prompt...");
    // In a live environment/recovery, we usually have 'konsole' or 'xterm'
    let _ = Command::new("konsole").spawn();
}

pub fn update_tre() -> Result<(), String> {
    println!("TRE: Checking for TRE updates...");
    let status = Command::new("pacman")
        .args(["-Sy", "tekipaki-tre", "--noconfirm"])
        .status();

    match status {
        Ok(s) if s.success() => Ok(()),
        _ => Err("TRE Update failed".to_string()),
    }
}
