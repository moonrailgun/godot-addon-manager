# Godot Addon Manager (gdam)

A package manager for Godot addons.

## Installation

```bash
cargo install godot-addon-manager
```

## Usage

```bash
gdam init       # Initialize gdam in current Godot project
gdam install    # Install all addons from gdam.yaml
gdam install <git-url>  # Install addon from git repository
gdam list       # List installed addons
gdam upgrade    # Upgrade installed addons
gdam version    # Show version information
```

## Development

```bash
# Install locally for development
cargo install --path .
```
