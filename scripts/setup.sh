#!/bin/bash

echo "Setting up notify.desktop development environment..."

# Install dependencies
echo "Installing npm dependencies..."
npm install

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if Tauri CLI is installed
if ! command -v tauri &> /dev/null; then
    echo "Installing Tauri CLI..."
    npm install -g @tauri-apps/cli
fi

echo "Setup complete!"
echo "Run 'npm run tauri:dev' to start development"