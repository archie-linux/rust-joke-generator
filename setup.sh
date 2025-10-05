#!/bin/bash
set -e

echo "Installing Python dependencies..."
pip3 install flask flask_bootstrap requests pync

echo "Building Rust backend..."
cargo build

echo "Setup complete!"
echo ""
echo "To run all components in the background, add this line to your crontab:"
echo "@reboot <path_to_[run_all.sh]_script>"
echo ""
echo "You can edit your crontab with: crontab -e"
