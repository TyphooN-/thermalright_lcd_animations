# Quick Start Guide

## 1. Prerequisites

A Rust toolchain (1.70+). Get one from [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 2. Build

```bash
cd thermalright_lcd_animations
cargo build --release
```

The binary is at `./target/release/thermalright-lcd`.

## 3. Run (no install needed)

```bash
# Interactive TUI — pick animations with arrow keys, space for random, q to quit
./target/release/thermalright-lcd

# List all available animations
./target/release/thermalright-lcd --list

# Run one animation continuously
./target/release/thermalright-lcd --animation rainbow_wave_ltr

# Faster timing
./target/release/thermalright-lcd --duration 5 --interval 0.01
```

## 4. Install (optional)

Adds a udev rule so the device is accessible without root, plus an optional systemd unit:

```bash
sudo ./install.sh
```

## Troubleshooting

### Permission denied opening the device

```bash
sudo ./install.sh    # installs the udev rule
# unplug and replug the USB cable, or reboot
```

### Device not found

- Verify the cooler's LCD cable is plugged in.
- Check `lsusb` shows `0416:8001` (Winbond — Thermalright uses Winbond's HID transfer block).
- If your device has a different VID/PID, set them in `config.json` or pass them as `--vendor`/`--product` flags after editing the source.

### Build error: `hidapi` couldn't link

Install the system `hidapi`/`libudev` development headers (Debian/Ubuntu: `sudo apt install libudev-dev pkg-config`).

## Configuration

`config.json` (optional):

```json
{
  "vendor_id": "0x0416",
  "product_id": "0x8001",
  "animation_mode": "interactive",
  "update_interval": 0.015,
  "rotation_duration": 10.0,
  "animations": ["rainbow_cycle", "aurora", "game_of_life"]
}
```

CLI flags always win over config values.
