#!/bin/bash

# Exit immediately if any command fails
set -e

CERT_DIR="./certs"
echo "--- Starting Local TLS/SSL Setup ---"

# 1. Detect if mkcert is installed, otherwise prompt the user
if ! command -v mkcert &> /dev/null; then
    echo "❌ Error: 'mkcert' is not installed."
    echo "Please install it using your package manager!"
    exit 1
else
    echo "[x] mkcert is already installed."
fi

# 2. Install the local Certificate Authority into browser root stores
echo "[x] Installing local CA into system and browser trust stores..."
mkcert -install

# 3. Create certificates directory if it doesn't exist
mkdir -p "$CERT_DIR"
cd "$CERT_DIR"

# 4. Generate the certificates for localhost and 127.0.0.1
echo "[x] Generating certificates for localhost and 127.0.0.1..."
mkcert localhost 127.0.0.1 ::1

# 5. Rename files to match Rust backend expectations
echo "[x] Renaming certificates to cert.pem and key.pem..."
mv localhost+2.pem cert.pem
mv localhost+2-key.pem key.pem

# 6. Safety check: Update .gitignore if needed
cd ..
if [ -f .gitignore ]; then
    if ! grep -q "key.pem" .gitignore; then
        echo "[x] Adding key.pem to .gitignore..."
        echo "" >> .gitignore
        echo "certs/" >> .gitignore
    else
        echo "[x] certs/ is already in .gitignore."
    fi
else
    echo "[x] Creating .gitignore and protecting certs.pem..."
    echo "# Local TLS private keys" > .gitignore
    echo "certs/" >> .gitignore
fi

echo "--- Setup Complete Successfully! ---"
