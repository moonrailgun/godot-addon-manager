#!/bin/sh
# Godot Addon Manager (gdam) installer
# Usage: curl -fsSL https://raw.githubusercontent.com/moonrailgun/godot-addon-manager/master/scripts/install.sh | sh

set -e

REPO="moonrailgun/godot-addon-manager"
BINARY_NAME="gdam"
INSTALL_DIR="${GDAM_INSTALL_DIR:-$HOME/.local/bin}"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info() {
    printf "${GREEN}[INFO]${NC} %s\n" "$1"
}

warn() {
    printf "${YELLOW}[WARN]${NC} %s\n" "$1"
}

error() {
    printf "${RED}[ERROR]${NC} %s\n" "$1"
    exit 1
}

detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux)
            OS_NAME="linux"
            ;;
        Darwin)
            OS_NAME="macos"
            ;;
        MINGW*|MSYS*|CYGWIN*|Windows_NT)
            OS_NAME="windows"
            ;;
        *)
            error "Unsupported operating system: $OS"
            ;;
    esac

    case "$ARCH" in
        x86_64|amd64)
            ARCH_NAME="x86_64"
            ;;
        aarch64|arm64)
            ARCH_NAME="aarch64"
            ;;
        *)
            error "Unsupported architecture: $ARCH"
            ;;
    esac

    PLATFORM="${OS_NAME}-${ARCH_NAME}"
    info "Detected platform: $PLATFORM"
}

get_latest_version() {
    if command -v curl >/dev/null 2>&1; then
        VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
    elif command -v wget >/dev/null 2>&1; then
        VERSION=$(wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/')
    else
        error "Neither curl nor wget found. Please install one of them."
    fi

    if [ -z "$VERSION" ]; then
        error "Failed to get latest version"
    fi

    info "Latest version: $VERSION"
}

download_and_install() {
    TMP_DIR=$(mktemp -d)

    if [ "$OS_NAME" = "windows" ]; then
        DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY_NAME}-${PLATFORM}.zip"
        ARCHIVE_PATH="${TMP_DIR}/${BINARY_NAME}.zip"
    else
        DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY_NAME}-${PLATFORM}.tar.gz"
        ARCHIVE_PATH="${TMP_DIR}/${BINARY_NAME}.tar.gz"
    fi

    info "Downloading from: $DOWNLOAD_URL"

    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$DOWNLOAD_URL" -o "$ARCHIVE_PATH" || error "Download failed"
    elif command -v wget >/dev/null 2>&1; then
        wget -q "$DOWNLOAD_URL" -O "$ARCHIVE_PATH" || error "Download failed"
    fi

    info "Extracting..."
    if [ "$OS_NAME" = "windows" ]; then
        unzip -o "$ARCHIVE_PATH" -d "$TMP_DIR" || error "Extraction failed"
        EXE_NAME="${BINARY_NAME}.exe"
    else
        tar -xzf "$ARCHIVE_PATH" -C "$TMP_DIR"
        EXE_NAME="${BINARY_NAME}"
    fi

    mkdir -p "$INSTALL_DIR"
    mv "${TMP_DIR}/${EXE_NAME}" "${INSTALL_DIR}/${EXE_NAME}"
    chmod +x "${INSTALL_DIR}/${EXE_NAME}"

    rm -rf "$TMP_DIR"
    info "Installed to: ${INSTALL_DIR}/${EXE_NAME}"
}

check_path() {
    case ":$PATH:" in
        *":$INSTALL_DIR:"*)
            ;;
        *)
            warn "Add the following to your shell profile (.bashrc, .zshrc, etc.):"
            echo ""
            echo "    export PATH=\"\$PATH:$INSTALL_DIR\""
            echo ""
            ;;
    esac
}

main() {
    echo ""
    echo "  ╔════════════════════════════════════════╗"
    echo "  ║   Godot Addon Manager (gdam) Installer ║"
    echo "  ╚════════════════════════════════════════╝"
    echo ""

    detect_platform
    get_latest_version
    download_and_install
    check_path

    echo ""
    info "Installation complete! Run 'gdam --help' to get started."
    echo ""
}

main
