#!/bin/bash

# Check for root privileges
if [[ $EUID -ne 0 ]]; then
    echo "This script must be run as root, as it will move the binary to /usr/local/bin" 1>&2
    exit 1
fi

# Detect system architecture
arch=$(uname -m)

case "$arch" in
    armv7*)
        target="armv7-unknown-linux-gnueabihf"
        ;;
    aarch64 | aarch64_be | armv8b | armv8l)
        target="aarch64-unknown-linux-gnu"
        ;;
    i686 | x86_64)
        target="x86_64-unknown-linux-gnu"
        ;;
    *)
        echo "Unsupported architecture: $arch"
        exit 1
        ;;
esac

# Download and install Swerve
curl -L https://github.com/SpikeHD/swerve/releases/latest/download/swerve-$target -o swerve

chmod +x swerve

# Move to local bin folder
mkdir -p /usr/local/bin
mv swerve /usr/local/bin
