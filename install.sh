#!/bin/sh
set -eu

# Configuration
REPO="${REPO:-0xNicolinux/nFiles}"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"
BINARY_NAME="file-organizer"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

log_info() {
    printf "${BLUE}==>${NC} %s\n" "$1"
}

log_success() {
    printf "${GREEN}==>${NC} %s\n" "$1"
}

log_warn() {
    printf "${YELLOW}Warning:${NC} %s\n" "$1"
}

log_error() {
    printf "${RED}Error:${NC} %s\n" "$1" >&2
}

# Require curl
if ! command -v curl >/dev/null 2>&1; then
    log_error "curl is required to install ${BINARY_NAME}."
    exit 1
fi

# Detect OS
OS_TYPE="$(uname -s)"
case "${OS_TYPE}" in
    Linux*)     TARGET_OS="unknown-linux-gnu";;
    Darwin*)    TARGET_OS="apple-darwin";;
    *)          log_error "Unsupported operating system: ${OS_TYPE}"; exit 1;;
esac

# Detect Architecture
ARCH_TYPE="$(uname -m)"
case "${ARCH_TYPE}" in
    x86_64|amd64)   TARGET_ARCH="x86_64";;
    aarch64|arm64)  TARGET_ARCH="aarch64";;
    *)              log_error "Unsupported CPU architecture: ${ARCH_TYPE}"; exit 1;;
esac

TARGET="${TARGET_ARCH}-${TARGET_OS}"
log_info "Detected target platform: ${TARGET}"

# Get latest release tag if not specified
if [ -z "${VERSION:-}" ]; then
    log_info "Fetching latest release version for ${REPO}..."
    LATEST_URL="https://api.github.com/repos/${REPO}/releases/latest"
    VERSION=$(curl -sSL "${LATEST_URL}" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

    if [ -z "${VERSION}" ]; then
        log_error "Could not determine latest release version from ${LATEST_URL}."
        log_error "Please check that ${REPO} has a published release."
        exit 1
    fi
fi

log_info "Installing ${BINARY_NAME} ${VERSION}..."

ASSET_NAME="${BINARY_NAME}-${VERSION}-${TARGET}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET_NAME}"

# Create temporary directory for download
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "${TMP_DIR}"' EXIT INT TERM

ARCHIVE_PATH="${TMP_DIR}/${ASSET_NAME}"

log_info "Downloading ${DOWNLOAD_URL}..."
if ! curl -fsSL "${DOWNLOAD_URL}" -o "${ARCHIVE_PATH}"; then
    log_error "Failed to download asset from ${DOWNLOAD_URL}."
    log_error "Please ensure that a binary asset for target '${TARGET}' exists for version '${VERSION}'."
    exit 1
fi

log_info "Extracting archive..."
tar -xzf "${ARCHIVE_PATH}" -C "${TMP_DIR}"

EXECUTABLE_PATH="${TMP_DIR}/${BINARY_NAME}"
if [ ! -f "${EXECUTABLE_PATH}" ]; then
    log_error "Downloaded archive does not contain expected binary '${BINARY_NAME}'."
    exit 1
fi

# Create installation directory if it doesn't exist
mkdir -p "${INSTALL_DIR}"

log_info "Installing binary to ${INSTALL_DIR}/${BINARY_NAME}..."
cp "${EXECUTABLE_PATH}" "${INSTALL_DIR}/${BINARY_NAME}"
chmod 755 "${INSTALL_DIR}/${BINARY_NAME}"

log_success "${BINARY_NAME} installed successfully."
echo ""
echo "Binary:"
echo "  ${INSTALL_DIR}/${BINARY_NAME}"
echo ""

# Check PATH
case ":${PATH}:" in
    *:"${INSTALL_DIR}":*)
        ;;
    *)
        log_warn "${INSTALL_DIR} is not in your PATH."
        echo "Add it to your PATH by adding this line to your shell configuration file (e.g. ~/.bashrc or ~/.zshrc):"
        echo ""
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
        echo ""
        ;;
esac

echo "Then run:"
echo ""
echo "  ${BINARY_NAME} --help"
echo ""
