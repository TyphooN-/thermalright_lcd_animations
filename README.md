# Thermalright LCD Animations

A creative animation showcase for the Thermalright Peerless Assassin LCD display. This project demonstrates the visual capabilities of the LCD with a variety of colorful animations and effects.

## Features

- **20+ Unique Animations** including waves, gradients, patterns, and effects
- **Full RGB Color Support** with smooth transitions
- **Auto-Rotation Mode** to cycle through all animations
- **Individual Animation Selection** for testing specific effects
- **Configurable Timing** for animation speed and rotation duration

## Animation Showcase

### Wave Animations
- Rainbow Wave (Left to Right)
- Rainbow Wave (Right to Left)
- Dual Wave (CPU and GPU sides)
- Ocean Wave (Blue gradient)
- Fire Wave (Red/Orange/Yellow)

### Pattern Animations
- Knight Rider Scanner
- Larson Scanner (Dual)
- Cylon Eye
- Chasing Lights
- Police Strobe
- Checkerboard
- Alternating Bars

### Effect Animations
- Color Breathing
- Rainbow Cycle
- Sparkle/Twinkle
- Random Color Burst
- Gradient Sweep
- Plasma Effect
- Matrix Rain

### Display Animations
- Binary Counter
- Segment Crawl
- Loading Bar
- Digital Clock with Effects
- Temperature Simulation

## Prerequisites

- Python 3.8+
- `hidapi` library
- Thermalright Peerless Assassin LCD cooler connected via USB

## Installation

1. Clone the repository:
```bash
git clone <repo-url>
cd thermalright_lcd_animations
```

2. Create virtual environment and install dependencies:
```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

3. Run the installation script (sets up udev rules):
```bash
sudo ./install.sh
```

## Usage

### Run Auto-Rotation Demo (cycles through all animations)
```bash
python src/animations.py
```

### Run Specific Animation
```bash
python src/animations.py --animation rainbow_wave
```

### List All Available Animations
```bash
python src/animations.py --list
```

### Configuration

Edit `config.json` to customize:
- Animation rotation duration
- Update interval (speed)
- Color palettes
- USB Vendor/Product IDs

## Available Animations

Run with `--list` to see all available animations.

## Hardware Support

- Peerless Assassin 120 Digital
- Peerless Assassin 140 Digital
- Other Thermalright USB LCD displays with compatible protocol

## License

MIT License
