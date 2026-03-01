#!/bin/sh
set -e

REPO="fastforgedev/fastforge"
BINARY_NAME="fastforge"
INSTALL_DIR="${FASTFORGE_INSTALL_DIR:-/usr/local/bin}"

# ── Terminal colors ────────────────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
RESET='\033[0m'

info()    { printf "${CYAN}info${RESET}  %s\n" "$1"; }
success() { printf "${GREEN}✓${RESET}     %s\n" "$1"; }
warn()    { printf "${YELLOW}warn${RESET}  %s\n" "$1"; }
error()   { printf "${RED}error${RESET} %s\n" "$1" >&2; exit 1; }

# ── Detect OS / arch ──────────────────────────────────────────────────────────
detect_platform() {
  OS="$(uname -s)"
  ARCH="$(uname -m)"

  case "$OS" in
    Linux)  OS_NAME="linux" ;;
    Darwin) OS_NAME="darwin" ;;
    *)      error "Unsupported operating system: $OS" ;;
  esac

  case "$ARCH" in
    x86_64 | amd64)  ARCH_NAME="x86_64" ;;
    aarch64 | arm64) ARCH_NAME="aarch64" ;;
    *) error "Unsupported architecture: $ARCH" ;;
  esac

  PLATFORM="${OS_NAME}-${ARCH_NAME}"
}

# ── Map platform to Rust target triple ────────────────────────────────────────
resolve_target() {
  case "$PLATFORM" in
    darwin-aarch64) TARGET="aarch64-apple-darwin" ;;
    darwin-x86_64)  TARGET="x86_64-apple-darwin" ;;
    linux-aarch64)  TARGET="aarch64-unknown-linux-gnu" ;;
    linux-x86_64)   TARGET="x86_64-unknown-linux-gnu" ;;
    *) error "No prebuilt binary available for platform: $PLATFORM" ;;
  esac
}

# ── Resolve latest version from GitHub ────────────────────────────────────────
resolve_version() {
  if [ -n "$FASTFORGE_VERSION" ]; then
    VERSION="$FASTFORGE_VERSION"
    info "Using specified version: $VERSION"
    return
  fi

  info "Fetching latest release version..."

  if command -v curl >/dev/null 2>&1; then
    RELEASE_JSON="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases")"
  elif command -v wget >/dev/null 2>&1; then
    RELEASE_JSON="$(wget -qO- "https://api.github.com/repos/${REPO}/releases")"
  else
    error "Neither curl nor wget is available. Please install one and retry."
  fi

  VERSION="$(printf '%s' "$RELEASE_JSON" | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"v\?\([^"]*\)".*/\1/')"

  if [ -z "$VERSION" ]; then
    error "Failed to resolve the latest version. Set FASTFORGE_VERSION to specify one manually."
  fi

  info "Latest version: $VERSION"
}

# ── Download ──────────────────────────────────────────────────────────────────
download() {
  ARCHIVE_NAME="${BINARY_NAME}-${VERSION}-${TARGET}.tar.gz"
  DOWNLOAD_URL="https://github.com/${REPO}/releases/download/v${VERSION}/${ARCHIVE_NAME}"

  TMP_DIR="$(mktemp -d)"
  ARCHIVE_PATH="${TMP_DIR}/${ARCHIVE_NAME}"

  info "Downloading ${ARCHIVE_NAME}..."
  info "  from ${DOWNLOAD_URL}"

  if command -v curl >/dev/null 2>&1; then
    curl -fsSL --progress-bar "$DOWNLOAD_URL" -o "$ARCHIVE_PATH" || \
      error "Download failed. Check your network or verify that version v${VERSION} exists."
  elif command -v wget >/dev/null 2>&1; then
    wget -q --show-progress "$DOWNLOAD_URL" -O "$ARCHIVE_PATH" 2>&1 || \
      error "Download failed. Check your network or verify that version v${VERSION} exists."
  fi

  success "Downloaded ${ARCHIVE_NAME}"
}

# ── Install ───────────────────────────────────────────────────────────────────
install_binary() {
  info "Extracting archive..."
  tar -xzf "$ARCHIVE_PATH" -C "$TMP_DIR"

  BINARY_PATH="${TMP_DIR}/${BINARY_NAME}-${VERSION}-${TARGET}/${BINARY_NAME}"

  if [ ! -f "$BINARY_PATH" ]; then
    error "Binary not found in archive at expected path: ${BINARY_PATH}"
  fi

  chmod +x "$BINARY_PATH"

  if [ ! -d "$INSTALL_DIR" ]; then
    info "Creating install directory: ${INSTALL_DIR}"
    mkdir -p "$INSTALL_DIR" 2>/dev/null || sudo mkdir -p "$INSTALL_DIR"
  fi

  if [ -w "$INSTALL_DIR" ]; then
    mv "$BINARY_PATH" "${INSTALL_DIR}/${BINARY_NAME}"
  else
    info "Requesting elevated permissions to install into ${INSTALL_DIR}..."
    sudo mv "$BINARY_PATH" "${INSTALL_DIR}/${BINARY_NAME}"
  fi

  rm -rf "$TMP_DIR"

  success "Installed ${BINARY_NAME} to ${INSTALL_DIR}/${BINARY_NAME}"
}

# ── Verify ────────────────────────────────────────────────────────────────────
verify() {
  if command -v "$BINARY_NAME" >/dev/null 2>&1; then
    INSTALLED_VERSION="$("${INSTALL_DIR}/${BINARY_NAME}" --version 2>/dev/null || true)"
    success "Installation verified: ${INSTALLED_VERSION}"
  else
    warn "${BINARY_NAME} is not on your PATH."
    warn "Add the following line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    warn "  export PATH=\"${INSTALL_DIR}:\$PATH\""
  fi
}

# ── Main ──────────────────────────────────────────────────────────────────────
main() {
  printf "\n${BOLD}fastforge installer${RESET}\n\n"

  detect_platform
  resolve_target
  resolve_version
  download
  install_binary
  verify

  printf "\n${BOLD}${GREEN}fastforge ${VERSION} installed successfully!${RESET}\n\n"
  printf "Run ${CYAN}fastforge --help${RESET} to get started.\n\n"
}

main
