# Quick Start Guide

## 1. Install Dependencies

```bash
cd thermalright_lcd_animations
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

## 2. Test Without Installing

```bash
# List all available animations
python src/animations.py --list

# Run auto-rotate demo (cycles through all animations)
python src/animations.py

# Run a specific animation
python src/animations.py --animation rainbow_wave_ltr

# Custom timing
python src/animations.py --duration 5 --interval 0.03
```

## 3. Install (Optional)

Sets up udev rules and optionally creates a systemd service:

```bash
sudo ./install.sh
```

## Animation List

- **rainbow_wave_ltr** - Rainbow wave moving left to right
- **rainbow_wave_rtl** - Rainbow wave moving right to left
- **dual_wave** - Dual rainbow waves on CPU and GPU sides
- **ocean_wave** - Ocean-themed wave animation
- **fire_wave** - Fire-themed wave animation
- **knight_rider** - Knight Rider / K.I.T.T. scanner effect
- **cylon_eye** - Cylon eye scanner (both sides)
- **larson_scanner_dual** - Dual Larson scanners on CPU and GPU
- **chasing_lights** - Chasing lights effect
- **theater_chase** - Theater marquee chase effect
- **police_strobe** - Police strobe light effect
- **checkerboard** - Checkerboard pattern
- **alternating_bars** - Alternating color bars
- **color_breathing** - Smooth breathing effect with color cycle
- **rainbow_cycle** - Rainbow color cycle all LEDs same color
- **sparkle** - Random sparkle/twinkle effect
- **random_burst** - Random color bursts
- **gradient_sweep** - Gradient sweep across display
- **plasma** - Plasma effect
- **matrix_rain** - Matrix-style rain effect
- **binary_counter** - Binary counter animation
- **segment_crawl** - Crawl through display segments
- **loading_bar** - Loading bar animation
- **color_wipe** - Color wipe effect
- **rainbow_segments** - Each segment region gets different rainbow color

## Troubleshooting

### Permission Denied
If you get permission errors accessing the USB device:
```bash
sudo ./install.sh  # Installs udev rules
# Then log out and back in
```

### Device Not Found
- Make sure the LCD is connected via USB
- Check vendor/product IDs in config.json match your device
- Try running `lsusb` to verify the device is detected

## Configuration

Edit `config.json` to customize:
- Which animations to include in rotation
- Rotation duration per animation
- Update interval (animation speed)
- USB vendor/product IDs
