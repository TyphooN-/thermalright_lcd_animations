#!/bin/bash
set -e

if [ "$EUID" -ne 0 ]; then
  echo "Please run as root (sudo ./uninstall.sh)"
  exit 1
fi

echo "Uninstalling Thermalright LCD Animations..."
echo "============================================"

SERVICE_FILE="/etc/systemd/system/thermalright-lcd-animations.service"
if [ -f "$SERVICE_FILE" ]; then
  echo "Stopping and disabling systemd service..."
  systemctl stop thermalright-lcd-animations.service || true
  systemctl disable thermalright-lcd-animations.service || true
  rm -f "$SERVICE_FILE"
  systemctl daemon-reload
  echo "✓ Service removed."
else
  echo "✓ No service found."
fi

UDEV_RULE_FILE="/etc/udev/rules.d/70-thermalright-lcd.rules"
if [ -f "$UDEV_RULE_FILE" ]; then
  echo "Removing udev rule..."
  rm -f "$UDEV_RULE_FILE"
  udevadm control --reload-rules
  udevadm trigger
  echo "✓ udev rule removed."
else
  echo "✓ No udev rule found."
fi

echo ""
echo "============================================"
echo "Uninstallation complete!"
echo "(The binary at ./target/release/thermalright-lcd is left in place — remove manually or run 'cargo clean' if you also want to delete build artifacts.)"
