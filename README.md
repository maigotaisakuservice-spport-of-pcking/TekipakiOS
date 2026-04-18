# TekipakiOS - Perfect Edition v1.6

Arch Linux-based OS with Windows 11 UI/UX, powered by custom `linux-tekipaki` kernel.

## Building the OS

The build process is fully automated via GitHub Actions using a **self-hosted runner**.

### Prerequisites for GitHub Actions
1.  **Self-hosted Runner**: Setup a self-hosted runner with Docker installed.
2.  **GitHub Secrets**: Add the following secret to your repository:
    -   `TEKIPAKI_PRIVATE_KEY`: The private key used for licensing and secure component signing.

### Manual Build
To build locally (requires Arch Linux with `archiso` and `base-devel`):
```bash
sudo ./build-iso.sh
```

## Features
- **Custom Kernel**: `linux-tekipaki` based on Zen with high-responsiveness patches.
- **Windows 11 UI**: KDE Plasma customized with centered taskbar and original assets.
- **Guard System**: Advanced licensing system with 'Death Penalty' protection.
- **Recovery Environment (TRE)**: Self-healing OS environment (F11 at boot).
- **AUR Integration**: Pre-installed `pamac-aur` and more.

## Development
Rust components are located in `tekipakios/src/`. ISO profile is in `tekipakios/iso-profile/`.