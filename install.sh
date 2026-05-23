#!/bin/bash
set -e

# Check for root
if [ "$EUID" -ne 0 ]; then
  echo "Please run as root (sudo ./install.sh)"
  exit 1
fi

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

echo "Installing Thermalright LCD Animations..."
echo "=========================================="

# Build the binary if not already built
BIN="${SCRIPT_DIR}/target/release/thermalright-lcd"
if [ ! -x "$BIN" ]; then
  if ! command -v cargo > /dev/null 2>&1; then
    echo "Error: cargo (Rust toolchain) not found."
    echo "Install via https://rustup.rs/ then re-run this script."
    exit 1
  fi
  echo "Building release binary (this may take a minute)..."
  sudo -u "${SUDO_USER:-$USER}" cargo build --release --manifest-path "${SCRIPT_DIR}/Cargo.toml"
fi
echo "✓ Binary present at $BIN"

# Create udev rule
UDEV_RULE_FILE="/etc/udev/rules.d/70-thermalright-lcd.rules"
if [ ! -f "$UDEV_RULE_FILE" ]; then
  echo "Creating udev rule at $UDEV_RULE_FILE"
  cat > "$UDEV_RULE_FILE" <<EOF
SUBSYSTEM=="usb", ATTRS{idVendor}=="0416", ATTRS{idProduct}=="8001", MODE="0666"
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="0416", ATTRS{idProduct}=="8001", TAG+="uaccess"
EOF
  udevadm control --reload-rules
  udevadm trigger
  echo "✓ udev rule created."
else
  echo "✓ udev rule already exists."
fi

# Systemd service (optional)
read -p "Create systemd service to run on boot? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
  SERVICE_FILE="/etc/systemd/system/thermalright-lcd-animations.service"
  echo "Creating systemd service at $SERVICE_FILE"

  TARGET_USER=${SUDO_USER:-$USER}

  cat > "$SERVICE_FILE" <<EOL
[Unit]
Description=Thermalright LCD Animations
After=network.target

[Service]
ExecStart=${BIN}
WorkingDirectory=${SCRIPT_DIR}
Restart=always
User=${TARGET_USER}

[Install]
WantedBy=multi-user.target
EOL

  echo "✓ Systemd service file created."

  systemctl daemon-reload
  systemctl enable thermalright-lcd-animations.service
  systemctl start thermalright-lcd-animations.service
  echo "✓ Service enabled and started."
fi

chmod +x "${SCRIPT_DIR}/install.sh"
chmod +x "${SCRIPT_DIR}/uninstall.sh"

echo ""
echo "=========================================="
echo "Installation complete!"
echo ""
echo "Usage:"
echo "  Interactive mode:    ${BIN}"
echo "  List animations:     ${BIN} --list"
echo "  Run one animation:   ${BIN} --animation rainbow_cycle"
echo "  Auto-rotate:         ${BIN} --duration 10"
echo ""
