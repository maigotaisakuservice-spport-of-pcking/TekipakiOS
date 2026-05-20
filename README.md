# TekipakiOS - Perfect Edition v1.6

Arch Linux-based OS with Windows 11 UI/UX, powered by custom `linux-tekipaki` kernel.

## 🚀 Building the OS

The build process is fully automated via GitHub Actions using standard **GitHub-hosted runners** (`ubuntu-latest`).

### Prerequisites for GitHub Actions
1.  **GitHub Secrets**: Add the following secret to your repository:
    -   `TEKIPAKI_PRIVATE_KEY`: The private key used for licensing and secure component signing.

### Manual Build
To build locally (requires Arch Linux with `archiso` and `base-devel`):
```bash
sudo ./build-iso.sh
```

### Permission Fix (Post-build)
If the build process is run inside a container (like GitHub Actions), some files in the workspace might be owned by root. Run this command to restore ownership to your user:
```bash
sudo chown -R $USER:$USER .
```

---

## 🛠 OS Installer (Calamares)

Tekipaki OS uses a customized **Calamares** installer for a seamless setup experience.

### Installer Mechanism
- **Custom Branding**: Located in `tekipakios/iso-profile/airootfs/usr/share/calamares/branding/tekipaki/`.
- **Windows 11 UI Style**: The installer UI is themed using a custom QSS stylesheet (`stylesheet.qss`) that provides centered layouts and Modern UI colors.
- **Hardware Injection**: During installation, a custom shell process (`calamares-config.yaml`) detects hardware (NVIDIA GPUs, Realtek/Broadcom wireless) and injects the appropriate DKMS drivers automatically.
- **Kernel Hooking**: Automatically runs `mkinitcpio -P` to ensure the `linux-tekipaki` kernel is correctly configured with the host's hardware.

---

## 🛡 Security Architecture

Tekipaki OS is designed with a multi-layered security approach.

### 1. Guard System & Licensing
The **Guard System** ensures the integrity and legal usage of Tekipaki OS.
- **Algorithm**: Uses modular exponentiation with a 25-character Base29 key. Valid keys must satisfy $f(K) \pmod n = 0$ where $n$ corresponds to the edition (Home=3, Pro=5, Enterprise=8).
- **Death Penalty**: If the license is found to be invalid or tampered with, the `tekipaki-guardd` daemon will:
    1. Display a 0x8004DEAD BSOD.
    2. Wipe the MBR/GPT of the system drive to prevent further unauthorized use.

### 2. Kernel-Level Protection
- **linux-tekipaki**: Built with Hardened Gentoo-style patches and Zen responsiveness.
- **Kernel Lockdown**: Prevents unauthorized modifications to the running kernel memory even by the root user (when in Secure Boot mode).

### 3. Network & Hardware Security
- **nftables**: Pre-configured firewall with a strict "default-deny" policy for incoming connections.
- **USBGuard**: Protects against BadUSB attacks by authorizing only known USB devices.
- **TPM 2.0 Integration**: Used for encrypting the DataLocker partition in the Recovery Environment.

### Keygen Tools
- **CLI**: `tekipakios/src/guard/tekipaki-keygen/` (Rust-based).
- **Web**: `keygen.html` in the repository root (JavaScript-based).

---

## 🩹 Tekipaki Recovery Environment (TRE)

Accessible by pressing **F11** at boot time.

- **Self-Healing**: Scans system binaries against signed manifests and restores corrupted files.
- **DataLocker**: A secure partition for sensitive data, accessible only via TRE.
- **Cloud Reinstall**: Downloads the latest Tekipaki OS image and performs a clean install while preserving User data.

---

## 💻 Usage & Getting Started

### Initial Login
- **Default User**: `tekipaki`
- **Password**: (Empty / No password set by default, prompted to set at first boot)

### UI/UX Tips
- **Centered Taskbar**: Like Windows 11, the taskbar is centered. You can change this in *Tekipaki Settings*.
- **C: Drive Emulation**: Your home directory is logically linked to `C:\Users\tekipaki` via OverlayFS for compatibility with certain wine-based applications.
- **AUR Support**: Use `pamac` (GUI) or `yay` (CLI) to install community packages.

---

## 📁 Repository Structure

- `tekipakios/src/`: Source code for Rust components (Settings, Guard, TRE, etc.).
- `tekipakios/iso-profile/`: Archiso profile for generating the ISO.
- `root/`: Website files (Landing page, Download, Support, Keygen).

---

## ⚖️ License & Compliance

### Open Source Basis
Tekipaki OS is a derivative work based on **Arch Linux**. We respect and adhere to the licenses of our upstream components:
- **Arch Linux / Archiso**: GPL.
- **Linux Kernel**: GPLv2.
- **KDE Plasma**: LGPL / GPL.
- **Rust Standard Library**: MIT / Apache 2.0.

### Custom Components
While the base system is open source, the following components are unique to Tekipaki OS:
- **Tekipaki AppSet & Guard**: Custom source code provided under the Tekipaki Software License (see `license.html`).
- **Branding & Assets**: All "Tekipaki" logos, wallpapers, and naming conventions are proprietary assets of the Tekipaki OS Project.

Usage of this OS implies acceptance of the EULA, which governs the use of these custom components and the licensing system.
