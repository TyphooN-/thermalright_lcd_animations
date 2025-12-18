#!/usr/bin/env python3
"""Thermalright LCD Animations - Main entry point."""
import argparse
import json
import os
import sys
import time
from lcd_controller import LCDController
from animation_library import ANIMATIONS


def load_config(config_path=None):
    """Load configuration from JSON file."""
    if config_path is None:
        config_path = os.path.join(
            os.path.dirname(os.path.dirname(__file__)),
            'config.json'
        )

    try:
        with open(config_path, 'r') as f:
            return json.load(f)
    except Exception as e:
        print(f"Error loading config: {e}")
        return get_default_config()


def get_default_config():
    """Get default configuration."""
    return {
        'vendor_id': '0x0416',
        'product_id': '0x8001',
        'animation_mode': 'auto_rotate',
        'rotation_duration': 10.0,
        'update_interval': 0.05,
        'animations': list(ANIMATIONS.keys())
    }


def list_animations():
    """Print all available animations."""
    print("\n Available Animations")
    print("=" * 50)
    for name in sorted(ANIMATIONS.keys()):
        anim_class = ANIMATIONS[name]
        doc = anim_class.__doc__ or "No description"
        print(f"  {name:25s} - {doc.strip()}")
    print("=" * 50)
    print(f"\nTotal: {len(ANIMATIONS)} animations\n")


def run_single_animation(lcd, animation_name, update_interval=0.05):
    """Run a single animation indefinitely."""
    if animation_name not in ANIMATIONS:
        print(f"Error: Animation '{animation_name}' not found.")
        list_animations()
        return

    print(f"\nRunning animation: {animation_name}")
    print("Press Ctrl+C to stop\n")

    anim_class = ANIMATIONS[animation_name]
    anim = anim_class(lcd)
    anim.reset()

    try:
        while True:
            anim.update()
            lcd.send_packets()
            time.sleep(update_interval)
    except KeyboardInterrupt:
        print("\nStopping animation...")
        lcd.clear()
        lcd.send_packets()


def run_auto_rotate(lcd, animations, rotation_duration=10.0, update_interval=0.05):
    """Automatically rotate through animations."""
    print("\n Auto-Rotate Mode")
    print("=" * 50)
    print(f"Rotation Duration: {rotation_duration}s")
    print(f"Update Interval: {update_interval}s")
    print(f"Animations: {len(animations)}")
    print("\nPress Ctrl+C to stop\n")

    current_anim_idx = 0
    frames_per_animation = int(rotation_duration / update_interval)

    try:
        while True:
            # Get current animation
            anim_name = animations[current_anim_idx]
            if anim_name not in ANIMATIONS:
                print(f"Warning: Animation '{anim_name}' not found, skipping.")
                current_anim_idx = (current_anim_idx + 1) % len(animations)
                continue

            print(f"[{current_anim_idx + 1}/{len(animations)}] {anim_name}")

            anim_class = ANIMATIONS[anim_name]
            anim = anim_class(lcd)
            anim.reset()

            # Run animation for specified duration
            for _ in range(frames_per_animation):
                anim.update()
                lcd.send_packets()
                time.sleep(update_interval)

            # Move to next animation
            current_anim_idx = (current_anim_idx + 1) % len(animations)

    except KeyboardInterrupt:
        print("\n\nStopping auto-rotate...")
        lcd.clear()
        lcd.send_packets()


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description='Thermalright LCD Animations',
        formatter_class=argparse.RawDescriptionHelpFormatter
    )

    parser.add_argument(
        '--config', '-c',
        help='Path to config file',
        default=None
    )

    parser.add_argument(
        '--animation', '-a',
        help='Run specific animation',
        default=None
    )

    parser.add_argument(
        '--list', '-l',
        action='store_true',
        help='List all available animations'
    )

    parser.add_argument(
        '--duration', '-d',
        type=float,
        help='Animation rotation duration (seconds)',
        default=None
    )

    parser.add_argument(
        '--interval', '-i',
        type=float,
        help='Update interval (seconds)',
        default=None
    )

    parser.add_argument(
        '--interactive',
        action='store_true',
        help='Run in interactive mode with keyboard controls'
    )

    args = parser.parse_args()

    # List animations and exit
    if args.list:
        list_animations()
        return 0

    # Load configuration
    config = load_config(args.config)

    # Override config with command line args
    if args.duration is not None:
        config['rotation_duration'] = args.duration
    if args.interval is not None:
        config['update_interval'] = args.interval

    # Initialize LCD controller
    vendor_id = int(config.get('vendor_id', '0x0416'), 16)
    product_id = int(config.get('product_id', '0x8001'), 16)

    print("\nInitializing LCD Controller...")
    print(f"Vendor ID: 0x{vendor_id:04x}")
    print(f"Product ID: 0x{product_id:04x}")

    lcd = LCDController(vendor_id=vendor_id, product_id=product_id)

    if lcd.dev is None:
        print("\nError: Could not connect to LCD device.")
        print("Make sure:")
        print("  1. The device is connected")
        print("  2. You have proper permissions (run sudo ./install.sh)")
        print("  3. The vendor/product IDs are correct")
        return 1

    print("Connected successfully!\n")

    # Run animation
    try:
        if args.animation:
            # Run single animation
            run_single_animation(
                lcd,
                args.animation,
                config['update_interval']
            )
        else:
            # Auto-rotate mode
            animations = config.get('animations', list(ANIMATIONS.keys()))
            run_auto_rotate(
                lcd,
                animations,
                config['rotation_duration'],
                config['update_interval']
            )
    finally:
        lcd.close()

    return 0


if __name__ == '__main__':
    sys.exit(main())
