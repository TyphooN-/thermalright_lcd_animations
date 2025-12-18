#!/bin/bash

# Check for root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root"
  exit
fi

echo "Uninstalling Thermalright LCD Animations..."
echo "============================================"

# Stop and disable the service
SERVICE_FILE="/etc/systemd/system/thermalright-lcd-animations.service"
if [ -f "$SERVICE_FILE" ]; then
  echo "Stopping and disabling systemd service..."
  systemctl stop thermalright-lcd-animations.service
  systemctl disable thermalright-lcd-animations.service

  echo "Removing systemd service file..."
  rm "$SERVICE_FILE"
  systemctl daemon-reload
  echo "✓ Service removed."
else
  echo "✓ No service found."
fi

# Remove the udev rule
UDEV_RULE_FILE="/etc/udev/rules.d/70-thermalright-lcd.rules"
if [ -f "$UDEV_RULE_FILE" ]; then
  echo "Removing udev rule..."
  rm "$UDEV_RULE_FILE"
  udevadm control --reload-rules
  udevadm trigger
  echo "✓ udev rule removed."
else
  echo "✓ No udev rule found."
fi

echo ""
echo "============================================"
echo "Uninstallation complete!"
