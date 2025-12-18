#!/bin/bash

# Check for root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root"
  exit
fi

# Get the directory of the script
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

echo "Installing Thermalright LCD Animations..."
echo "=========================================="

# Create udev rule
UDEV_RULE_FILE="/etc/udev/rules.d/70-thermalright-lcd.rules"
if [ ! -f "$UDEV_RULE_FILE" ]; then
  echo "Creating udev rule at $UDEV_RULE_FILE"
  echo 'SUBSYSTEM=="usb", ATTRS{idVendor}=="0416", ATTRS{idProduct}=="8001", MODE="0666"' > "$UDEV_RULE_FILE"
  echo 'SUBSYSTEM=="hidraw", ATTRS{idVendor}=="0416", ATTRS{idProduct}=="8001", TAG+="uaccess"' >> "$UDEV_RULE_FILE"
  udevadm control --reload-rules
  udevadm trigger
  echo "✓ udev rule created."
else
  echo "✓ udev rule already exists."
fi

# Create systemd service (optional)
read -p "Create systemd service to run on boot? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
  SERVICE_FILE="/etc/systemd/system/thermalright-lcd-animations.service"
  echo "Creating systemd service at $SERVICE_FILE"

  # Get the user who ran sudo
  SUDO_USER=${SUDO_USER:-$USER}

  cat > "$SERVICE_FILE" << EOL
[Unit]
Description=Thermalright LCD Animations
After=network.target

[Service]
ExecStart=${SCRIPT_DIR}/.venv/bin/python ${SCRIPT_DIR}/src/animations.py
WorkingDirectory=${SCRIPT_DIR}
Restart=always
User=${SUDO_USER}

[Install]
WantedBy=multi-user.target
EOL

  echo "✓ Systemd service file created."

  # Reload systemd, enable and start the service
  echo "Enabling and starting the service..."
  systemctl daemon-reload
  systemctl enable thermalright-lcd-animations.service
  systemctl start thermalright-lcd-animations.service

  echo "✓ Service enabled and started."
fi

# Make scripts executable
chmod +x "${SCRIPT_DIR}/install.sh"
chmod +x "${SCRIPT_DIR}/uninstall.sh"
chmod +x "${SCRIPT_DIR}/src/animations.py"

echo ""
echo "=========================================="
echo "Installation complete!"
echo ""
echo "Usage:"
echo "  Run animations:     python src/animations.py"
echo "  List animations:    python src/animations.py --list"
echo "  Specific animation: python src/animations.py --animation knight_rider"
echo ""
echo "Have fun! ✨"
