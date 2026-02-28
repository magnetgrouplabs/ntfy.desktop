#!/bin/bash

# Date-based Versioning Script for ntfy.desktop
# Format: YYYY.MM.DD.build (e.g., 2026.02.27.1)

set -e

# Configuration
VERSION_FILE="scripts/last_version.txt"
BUILD_LOG="scripts/build_log.txt"

# Get current date in YYYY.MM.DD format
CURRENT_DATE=$(date +%Y.%m.%d)

# Function to get next build number
get_next_build_number() {
    local date=$1
    
    # Check if we have a build log file
    if [ -f "$BUILD_LOG" ]; then
        # Get the last build number for this date
        local last_build=$(grep "^$date:" "$BUILD_LOG" | tail -1 | cut -d: -f2)
        if [ -n "$last_build" ]; then
            echo $((last_build + 1))
            return
        fi
    fi
    
    # No builds for this date yet, start with 1
    echo 1
}

# Function to update version files
update_version_files() {
    local version=$1
    
    # Use simple semver for all files
    local semver_version="$version"
    
    # Update package.json
    sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$semver_version\"/" package.json
    rm -f package.json.bak
    
    # Update Cargo.toml
    if [ -f "src-tauri/Cargo.toml" ]; then
        sed -i.bak "s/^version = \"[^\"]*\"$/version = \"$semver_version\"/" src-tauri/Cargo.toml
        rm -f src-tauri/Cargo.toml.bak
    fi
    
    # Update tauri.conf.json
    if [ -f "src-tauri/tauri.conf.json" ]; then
        sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$semver_version\"/" src-tauri/tauri.conf.json
        rm -f src-tauri/tauri.conf.json.bak
    fi
}

# Function to log the build
log_build() {
    local version=$1
    local date=$2
    local build_number=$3
    
    # Create build log directory if it doesn't exist
    mkdir -p "$(dirname "$BUILD_LOG")"
    
    # Append to build log
    echo "$date:$build_number:$version" >> "$BUILD_LOG"
}

# Function to get current version
get_current_version() {
    if [ -f "package.json" ]; then
        grep '"version"' package.json | head -1 | sed 's/.*"version": "\([^"]*\)".*/\1/'
    fi
}

# Main versioning function
update_version() {
    local is_release=$1
    local force_version=$2
    
    # If force version is provided, use it
    if [ -n "$force_version" ]; then
        echo "Forcing version: $force_version"
        update_version_files "$force_version"
        return
    fi
    
    # Only update version for release builds
    if [ "$is_release" != "true" ]; then
        echo "Development build - keeping current version"
        return
    fi
    
    # Get next build number
    local build_number=$(get_next_build_number "$CURRENT_DATE")
    local new_version="${CURRENT_DATE}.${build_number}"
    
    echo "Release build detected"
    echo "Date: $CURRENT_DATE"
    echo "Build number: $build_number"
    echo "New version: $new_version"
    
    # Update version files
    update_version_files "$new_version"
    
    # Log the build
    log_build "$new_version" "$CURRENT_DATE" "$build_number"
    
    echo "Version updated to $new_version"
}

# Parse command line arguments
IS_RELEASE=false
FORCE_VERSION=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --release)
            IS_RELEASE=true
            shift
            ;;
        --force-version)
            FORCE_VERSION="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --release           Update version for release build"
            echo "  --force-version V   Force a specific version"
            echo "  -h, --help         Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Get current version
CURRENT_VERSION=$(get_current_version)
echo "Current version: $CURRENT_VERSION"

# Update version
update_version "$IS_RELEASE" "$FORCE_VERSION"

# Show new version
NEW_VERSION=$(get_current_version)
echo "New version: $NEW_VERSION"