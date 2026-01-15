# Introduction

**Godot Addon Manager (gdam)** is a package manager for Godot addons. It helps you manage, install, and update addons in your Godot projects with ease.

## Features

- **Easy Installation**: Install addons directly from Git repositories
- **Version Control**: Track addon versions and checksums for reproducible builds
- **Automatic Updates**: Upgrade addons to their latest versions
- **Cache Management**: Efficient caching system to speed up installations
- **Git Integration**: Automatically updates `.gitignore` to exclude managed addons

## Quick Start

```bash
# Initialize gdam in your Godot project
gdam init

# Install an addon from a Git repository
gdam install https://github.com/user/addon-repo

# List installed addons
gdam list

# Upgrade all addons
gdam upgrade
```

<!-- ## Installation

Install gdam using Cargo:

```bash
cargo install godot-addon-manager
```

## Requirements

- Rust (for installation via Cargo)
- Git (for cloning addon repositories)
- A valid Godot project (with `project.godot` file) -->
