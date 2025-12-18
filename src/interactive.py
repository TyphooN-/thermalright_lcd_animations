#!/usr/bin/env python3
"""Interactive mode for Thermalright LCD Animations with keyboard controls."""
import sys
import time
import random
import tty
import termios
import select
from lcd_controller import LCDController
from animation_library import ANIMATIONS


class InteractiveController:
    """Interactive animation controller with keyboard navigation."""

    def __init__(self, lcd, config):
        self.lcd = lcd
        self.config = config
        self.animation_names = list(ANIMATIONS.keys())
        self.current_index = 0
        self.current_anim = None
        self.duration = 10.0  # Default duration in seconds
        self.update_interval = config.get('update_interval', 0.005)
        self.random_mode = True
        self.start_time = time.time()

    def get_key(self):
        """Non-blocking keyboard input."""
        if select.select([sys.stdin], [], [], 0)[0]:
            return sys.stdin.read(1)
        return None

    def draw_ui(self):
        """Draw the status UI."""
        print("\033[2J\033[H", end='')  # Clear screen
        print("=" * 70)
        print(" THERMALRIGHT LCD ANIMATIONS - Interactive Mode")
        print(" (TyphooN@marketwizardry.org)")
        print("=" * 70)
        print()
        print(f" Current Animation: [{self.current_index + 1}/{len(self.animation_names)}]")
        print(f" {self.animation_names[self.current_index]}")
        print()
        print(f" Mode: {'RANDOM' if self.random_mode else 'MANUAL'}")
        print(f" Duration: {self.duration:.1f}s")
        print(f" Speed (interval): {self.update_interval:.3f}s")
        print()
        print("-" * 70)
        print(" Controls:")
        print("   ← →  : Previous/Next animation")
        print("   SPACE: Jump to random animation")
        print("   m    : Toggle Manual/Random mode")
        print("   + -  : Increase/Decrease duration (±1s)")
        print("   [ ]  : Decrease/Increase speed")
        print("   q    : Quit")
        print("-" * 70)
        print()

    def next_animation(self):
        """Go to next animation."""
        self.current_index = (self.current_index + 1) % len(self.animation_names)
        self.load_current_animation()

    def prev_animation(self):
        """Go to previous animation."""
        self.current_index = (self.current_index - 1) % len(self.animation_names)
        self.load_current_animation()

    def random_animation(self):
        """Select random animation."""
        self.current_index = random.randint(0, len(self.animation_names) - 1)
        self.load_current_animation()

    def load_current_animation(self):
        """Load the current animation."""
        anim_name = self.animation_names[self.current_index]
        anim_class = ANIMATIONS[anim_name]
        self.current_anim = anim_class(self.lcd)
        self.current_anim.reset()
        self.start_time = time.time()
        self.draw_ui()

    def adjust_duration(self, delta):
        """Adjust duration by delta seconds."""
        self.duration = max(1.0, self.duration + delta)
        self.draw_ui()

    def adjust_speed(self, faster=True):
        """Adjust animation speed."""
        if faster:
            self.update_interval = max(0.001, self.update_interval - 0.005)
        else:
            self.update_interval = min(0.2, self.update_interval + 0.005)
        self.draw_ui()

    def toggle_random(self):
        """Toggle random mode."""
        self.random_mode = not self.random_mode
        self.draw_ui()

    def run(self):
        """Run the interactive mode."""
        # Set up terminal for raw input
        old_settings = termios.tcgetattr(sys.stdin)
        try:
            tty.setcbreak(sys.stdin.fileno())

            # Load first animation
            self.load_current_animation()

            while True:
                # Check for keyboard input
                key = self.get_key()

                if key:
                    if key == 'q':
                        break
                    elif key == '\x1b':  # Arrow keys start with escape
                        # Read the rest of the arrow key sequence
                        next1 = sys.stdin.read(1)
                        next2 = sys.stdin.read(1)
                        if next1 == '[':
                            if next2 == 'C':  # Right arrow
                                self.next_animation()
                            elif next2 == 'D':  # Left arrow
                                self.prev_animation()
                    elif key == ' ':  # Spacebar
                        self.random_animation()
                    elif key == 'm' or key == 'M':  # Toggle mode
                        self.toggle_random()
                    elif key == '+' or key == '=':
                        self.adjust_duration(1.0)
                    elif key == '-' or key == '_':
                        self.adjust_duration(-1.0)
                    elif key == '[':
                        self.adjust_speed(faster=False)
                    elif key == ']':
                        self.adjust_speed(faster=True)

                # Check if duration expired in random mode
                if self.random_mode:
                    if time.time() - self.start_time >= self.duration:
                        self.random_animation()

                # Update animation
                if self.current_anim:
                    self.current_anim.update()
                    self.lcd.send_packets()

                time.sleep(self.update_interval)

        finally:
            # Restore terminal settings
            termios.tcsetattr(sys.stdin, termios.TCSADRAIN, old_settings)
            # Clear screen and show cursor
            print("\033[2J\033[H", end='')
            print("\nExiting interactive mode...")


def main():
    """Main entry point for interactive mode."""
    import json
    import os

    # Load config
    config_path = os.path.join(
        os.path.dirname(os.path.dirname(__file__)),
        'config.json'
    )

    try:
        with open(config_path, 'r') as f:
            config = json.load(f)
    except Exception as e:
        print(f"Error loading config: {e}")
        config = {}

    # Initialize LCD
    vendor_id = int(config.get('vendor_id', '0x0416'), 16)
    product_id = int(config.get('product_id', '0x8001'), 16)

    print("\nInitializing LCD Controller...")
    lcd = LCDController(vendor_id=vendor_id, product_id=product_id)

    if lcd.dev is None:
        print("\nError: Could not connect to LCD device.")
        return 1

    print("Connected successfully!\n")
    print("Starting interactive mode in 2 seconds...")
    time.sleep(2)

    # Run interactive controller
    controller = InteractiveController(lcd, config)
    try:
        controller.run()
    finally:
        lcd.clear()
        lcd.send_packets()
        lcd.close()

    return 0


if __name__ == '__main__':
    sys.exit(main())
