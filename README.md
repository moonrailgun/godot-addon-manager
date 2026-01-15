# Godot Addon Manager (gdam)

A package manager for Godot addons.

## Installation

### Quick Install (Recommended)

The easiest way to install gdam is using the installation script, which automatically downloads the latest release for your platform.

**macOS / Linux:**

```bash
curl -fsSL https://raw.githubusercontent.com/moonrailgun/godot-addon-manager/master/scripts/install.sh | sh
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/moonrailgun/godot-addon-manager/master/scripts/install.ps1 | iex
```

### Install via Cargo

If you have Rust installed:

```bash
cargo install godot-addon-manager
```

### Manual Download

Download the latest release from [GitHub Releases](https://github.com/moonrailgun/godot-addon-manager/releases).

## Usage

```bash
gdam init                   # Initialize gdam in current Godot project
gdam install                # Install all addons from gdam.yaml
gdam install <git-url>      # Install addon from git repository
gdam uninstall <name>       # Uninstall an addon
gdam list                   # List installed addons
gdam upgrade                # Upgrade all addons to latest version
gdam upgrade <name>         # Upgrade specific addon
gdam cache path             # Print cache directory path
gdam cache clear            # Clear cache directory
gdam version                # Show version information
```

## Configuration

gdam uses a `gdam.yaml` file to track installed addons:

```yaml
version: 1
addons:
  - name: my-addon
    version: "1.0.0"
    source: https://github.com/user/repo
    checksum: abc123...
```

## Development

```bash
# Install locally for development
cargo install --path .
```

### Release new version

```bash
cargo release [patch|minor|major] --execute
```
