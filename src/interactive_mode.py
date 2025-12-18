#!/usr/bin/env python3
"""Interactive keyboard controls for animation playback."""
import sys
import termios
import tty
import select
import time
import random


class KeyboardController:
    """Non-blocking keyboard input for Linux terminals."""

    def __init__(self):
        self.old_settings = None
        if sys.stdin.isatty():
            self.old_settings = termios.tcgetattr(sys.stdin)
            tty.setcbreak(sys.stdin.fileno())

    def get_key(self):
        """Get a key press without blocking. Returns None if no key pressed."""
        if select.select([sys.stdin], [], [], 0)[0]:
            key = sys.stdin.read(1)

            # Handle arrow keys (escape sequences)
            if key == '\x1b':
                # Read next two characters for arrow keys
                if select.select([sys.stdin], [], [], 0.1)[0]:
                    key += sys.stdin.read(2)

            return key
        return None

    def restore(self):
        """Restore terminal settings."""
        if self.old_settings:
            termios.tcsetattr(sys.stdin, termios.TCSADRAIN, self.old_settings)

    def __del__(self):
        """Cleanup on deletion."""
        self.restore()


def run_interactive_mode(lcd, animations_list, initial_duration=10.0, initial_interval=0.05):
    """
    Run animations in interactive mode with keyboard controls.

    Controls:
        Right Arrow / n   : Next animation
        Left Arrow / p    : Previous animation
        Space             : Random animation
        + / =             : Increase duration (+1s)
        - / _             : Decrease duration (-1s)
        [ / {             : Slower animation (increase interval)
        ] / }             : Faster animation (decrease interval)
        q / ESC           : Quit
    """
    from lcd_controller import LCDController
    from animation_library import ANIMATIONS

    kb = KeyboardController()

    current_idx = 0
    duration = initial_duration
    update_interval = initial_interval
    random_mode = False

    # Stats
    frame_count = 0
    start_time = time.time()
    last_switch_time = start_time

    print("\n" + "=" * 70)
    print(" 🎮 INTERACTIVE MODE")
    print("=" * 70)
    print("\n Controls:")
    print("   →/n        Next animation")
    print("   ←/p        Previous animation")
    print("   SPACE      Random animation")
    print("   +/=        Increase duration (+1s)")
    print("   -/_        Decrease duration (-1s)")
    print("   [/{        Slower (increase interval by 0.01s)")
    print("   ]/}        Faster (decrease interval by 0.01s)")
    print("   q/ESC      Quit")
    print("\n" + "=" * 70 + "\n")

    def print_status():
        """Print current status."""
        anim_name = animations_list[current_idx]
        elapsed = time.time() - last_switch_time
        remaining = max(0, duration - elapsed)

        mode_str = "🎲 RANDOM" if random_mode else "📋 MANUAL"

        print(f"\r[{current_idx + 1}/{len(animations_list)}] {anim_name:25s} | "
              f"{mode_str} | Duration: {duration:2.0f}s | Remaining: {remaining:4.1f}s | "
              f"Speed: {update_interval:.3f}s | FPS: {1/update_interval if update_interval > 0 else 0:.1f}",
              end='', flush=True)

    try:
        while True:
            # Get current animation
            anim_name = animations_list[current_idx]
            if anim_name not in ANIMATIONS:
                current_idx = (current_idx + 1) % len(animations_list)
                continue

            anim_class = ANIMATIONS[anim_name]
            anim = anim_class(lcd)
            anim.reset()

            last_switch_time = time.time()

            # Run current animation
            while True:
                # Check for key press
                key = kb.get_key()

                if key:
                    # Navigation
                    if key == '\x1b[C' or key == 'n':  # Right arrow or 'n'
                        current_idx = (current_idx + 1) % len(animations_list)
                        random_mode = False
                        break
                    elif key == '\x1b[D' or key == 'p':  # Left arrow or 'p'
                        current_idx = (current_idx - 1) % len(animations_list)
                        random_mode = False
                        break
                    elif key == ' ':  # Space - random
                        current_idx = random.randint(0, len(animations_list) - 1)
                        random_mode = True
                        break

                    # Duration control
                    elif key in ['+', '=']:
                        duration = min(60, duration + 1)
                    elif key in ['-', '_']:
                        duration = max(1, duration - 1)

                    # Speed control
                    elif key in ['[', '{']:
                        update_interval = min(0.2, update_interval + 0.01)
                    elif key in [']', '}']:
                        update_interval = max(0.01, update_interval - 0.01)

                    # Quit
                    elif key == 'q' or key == '\x1b':  # 'q' or ESC
                        print("\n\nExiting interactive mode...")
                        return

                # Update animation
                anim.update()
                lcd.send_packets()
                frame_count += 1

                # Print status
                print_status()

                # Check if duration elapsed (only in random mode)
                if random_mode and (time.time() - last_switch_time) >= duration:
                    current_idx = random.randint(0, len(animations_list) - 1)
                    break

                # Sleep
                time.sleep(update_interval)

    except KeyboardInterrupt:
        print("\n\nInterrupted by user...")
    finally:
        kb.restore()
        lcd.clear()
        lcd.send_packets()

        # Print final stats
        total_time = time.time() - start_time
        print(f"\n\n Statistics:")
        print(f"   Total time: {total_time:.1f}s")
        print(f"   Total frames: {frame_count}")
        print(f"   Average FPS: {frame_count / total_time:.1f}")
        print()
