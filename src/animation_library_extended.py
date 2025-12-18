"""Extended animation library with additional creative effects."""
import math
import random
import numpy as np
from color_utils import (
    interpolate_color, wheel, random_color, hsv_to_rgb,
    get_gradient, get_palette
)
from led_map import LED_REGIONS, NUMBER_OF_LEDS


class Animation:
    """Base class for LED animations."""

    def __init__(self, lcd):
        self.lcd = lcd
        self.frame = 0
        self.name = self.__class__.__name__

    def update(self):
        """Update animation state (called each frame)."""
        self.frame += 1

    def reset(self):
        """Reset animation to initial state."""
        self.frame = 0
        self.lcd.clear()


# ============================================================================
# PULSE & STROBE ANIMATIONS
# ============================================================================

class Heartbeat(Animation):
    """Heartbeat pulse effect."""

    def update(self):
        self.lcd.set_all_leds(1)
        # Two pulses close together, then pause
        t = (self.frame % 100) / 100.0

        if t < 0.1:  # First beat
            brightness = math.sin(t * 10 * math.pi)
        elif 0.2 < t < 0.3:  # Second beat
            brightness = math.sin((t - 0.2) * 10 * math.pi)
        else:  # Rest
            brightness = 0.1

        color = hsv_to_rgb(0, 1.0, brightness)
        self.lcd.set_all_colors(color)
        super().update()


class Lighthouse(Animation):
    """Rotating lighthouse beacon."""

    def update(self):
        self.lcd.clear()
        beam_width = 10
        center = (self.frame * 2) % NUMBER_OF_LEDS

        for i in range(beam_width):
            idx = (center + i - beam_width // 2) % NUMBER_OF_LEDS
            distance_from_center = abs(i - beam_width // 2)
            brightness = 1.0 - (distance_from_center / (beam_width / 2))

            self.lcd.set_led(idx, 1)
            self.lcd.set_color(idx, hsv_to_rgb(50, 0.8, brightness))

        super().update()


class EmergencyStrobe(Animation):
    """Emergency vehicle strobe pattern."""

    def update(self):
        self.lcd.clear()
        pattern = (self.frame // 3) % 8

        if pattern < 3:  # Left red strobes
            self.lcd.set_leds(LED_REGIONS['cpu'], 1)
            for i in LED_REGIONS['cpu']:
                self.lcd.set_color(i, 'ff0000')
        elif pattern < 6:  # Right blue strobes
            self.lcd.set_leds(LED_REGIONS['gpu'], 1)
            for i in LED_REGIONS['gpu']:
                self.lcd.set_color(i, '0000ff')
        # Gap on pattern 6-7

        super().update()


class MorseCodeSOS(Animation):
    """Morse code SOS signal."""

    def __init__(self, lcd):
        super().__init__(lcd)
        # S = ... O = --- S = ...
        # Dot=1 frame, Dash=3 frames, space=1, letter gap=3
        self.pattern = [1,0,1,0,1,0,0,0,  # S
                       3,0,3,0,3,0,0,0,  # O
                       1,0,1,0,1,0,0,0,0,0]  # S + gap

    def update(self):
        idx = self.frame % len(self.pattern)

        if self.pattern[idx] > 0:
            self.lcd.set_all_leds(1)
            self.lcd.set_all_colors('ffffff')
        else:
            self.lcd.clear()

        super().update()


class StrobeMulticolor(Animation):
    """Multi-color strobe effect."""

    def update(self):
        if self.frame % 10 < 3:
            self.lcd.set_all_leds(1)
            hue = random.randint(0, 360)
            self.lcd.set_all_colors(hsv_to_rgb(hue, 1.0, 1.0))
        else:
            self.lcd.clear()

        super().update()


# ============================================================================
# GEOMETRIC PATTERNS
# ============================================================================

class Snake(Animation):
    """Snake crawling through LEDs."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.snake_length = 20

    def update(self):
        self.lcd.clear()
        head = self.frame % NUMBER_OF_LEDS

        for i in range(self.snake_length):
            idx = (head - i) % NUMBER_OF_LEDS
            brightness = 1.0 - (i / self.snake_length)
            hue = (self.frame + i * 5) % 360

            self.lcd.set_led(idx, 1)
            self.lcd.set_color(idx, hsv_to_rgb(hue, 1.0, brightness))

        super().update()


class BouncingBall(Animation):
    """Bouncing ball with physics that loops continuously."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.position = 0
        self.velocity = 2
        self.gravity = 0.2

    def update(self):
        self.lcd.clear()

        # Physics
        self.velocity += self.gravity
        self.position += self.velocity

        # Bounce
        if self.position >= NUMBER_OF_LEDS - 1:
            self.position = NUMBER_OF_LEDS - 1
            self.velocity = -self.velocity * 0.85  # Energy loss

        if self.position < 0:
            self.position = 0
            self.velocity = -self.velocity

        # Reset ball when velocity gets too low (loop forever)
        if abs(self.velocity) < 0.5:
            self.position = random.randint(0, NUMBER_OF_LEDS - 20)
            self.velocity = random.uniform(1.5, 3.0)

        # Draw ball with trail
        ball_size = 5
        for i in range(ball_size):
            idx = int(self.position) - i
            if 0 <= idx < NUMBER_OF_LEDS:
                brightness = 1.0 - (i / ball_size)
                hue = (self.frame + i * 10) % 360  # Add color variety
                self.lcd.set_led(idx, 1)
                self.lcd.set_color(idx, hsv_to_rgb(hue, 1.0, brightness))

        super().update()


class PingPong(Animation):
    """Ping pong between CPU and GPU sides."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.ball_pos = 0
        self.ball_side = 'cpu'
        self.ball_vel = 2

    def update(self):
        self.lcd.clear()

        # Update ball position
        self.ball_pos += self.ball_vel

        # Bounce logic
        if self.ball_pos >= 41 and self.ball_side == 'cpu':
            self.ball_side = 'gpu'
            self.ball_pos = 0
        elif self.ball_pos >= 41 and self.ball_side == 'gpu':
            self.ball_side = 'cpu'
            self.ball_pos = 0

        # Draw ball
        region = LED_REGIONS['cpu'] if self.ball_side == 'cpu' else LED_REGIONS['gpu']
        if 0 <= self.ball_pos < len(region):
            led_idx = region[int(self.ball_pos)]
            color = 'ffff00' if self.ball_side == 'cpu' else '00ffff'
            self.lcd.set_led(led_idx, 1)
            self.lcd.set_color(led_idx, color)

        super().update()


class Spiral(Animation):
    """Spiral pattern."""

    def update(self):
        self.lcd.set_all_leds(1)

        for i in range(NUMBER_OF_LEDS):
            angle = (i / NUMBER_OF_LEDS) * 2 * math.pi
            offset = math.sin(angle * 3 + self.frame * 0.1) * 0.5 + 0.5
            hue = int(offset * 360)

            self.lcd.set_color(i, hsv_to_rgb(hue, 1.0, 1.0))

        super().update()


# ============================================================================
# AUDIO/MUSIC INSPIRED
# ============================================================================

class VUMeter(Animation):
    """VU meter simulation."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.level_cpu = 0
        self.level_gpu = 0

    def update(self):
        self.lcd.clear()

        # Simulate audio levels
        self.level_cpu = abs(math.sin(self.frame * 0.15)) * 40
        self.level_gpu = abs(math.sin(self.frame * 0.12 + 1)) * 40

        # Draw CPU meter
        for i in range(int(self.level_cpu)):
            if i < len(LED_REGIONS['cpu']):
                led_idx = LED_REGIONS['cpu'][i]
                # Green -> Yellow -> Red gradient
                if i < 25:
                    color = '00ff00'
                elif i < 35:
                    color = 'ffff00'
                else:
                    color = 'ff0000'
                self.lcd.set_led(led_idx, 1)
                self.lcd.set_color(led_idx, color)

        # Draw GPU meter
        for i in range(int(self.level_gpu)):
            if i < len(LED_REGIONS['gpu']):
                led_idx = LED_REGIONS['gpu'][i]
                if i < 25:
                    color = '00ff00'
                elif i < 35:
                    color = 'ffff00'
                else:
                    color = 'ff0000'
                self.lcd.set_led(led_idx, 1)
                self.lcd.set_color(led_idx, color)

        super().update()


class Equalizer(Animation):
    """Equalizer bars simulation."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.num_bands = 7
        self.band_heights = [0] * self.num_bands

    def update(self):
        self.lcd.clear()

        # Update band heights
        for i in range(self.num_bands):
            target = abs(math.sin(self.frame * 0.1 + i * 0.5)) * 10
            self.band_heights[i] = self.band_heights[i] * 0.7 + target * 0.3

        # Draw bands
        leds_per_band = NUMBER_OF_LEDS // self.num_bands
        for band in range(self.num_bands):
            height = int(self.band_heights[band])
            start_led = band * leds_per_band

            for i in range(height):
                led_idx = start_led + i
                if led_idx < NUMBER_OF_LEDS:
                    hue = (band * 50 + self.frame) % 360
                    self.lcd.set_led(led_idx, 1)
                    self.lcd.set_color(led_idx, hsv_to_rgb(hue, 1.0, 1.0))

        super().update()


class BeatPulse(Animation):
    """Beat pulse synchronized effect."""

    def update(self):
        self.lcd.set_all_leds(1)

        # Simulate beat every 30 frames
        beat_cycle = self.frame % 30

        if beat_cycle < 5:
            brightness = 1.0 - (beat_cycle / 5.0)
            scale = 1.0 + brightness * 0.5
        else:
            brightness = 0.3
            scale = 1.0

        hue = (self.frame * 2) % 360
        color = hsv_to_rgb(hue, 1.0, brightness)
        self.lcd.set_all_colors(color)

        super().update()


# ============================================================================
# NATURE INSPIRED
# ============================================================================

class Lightning(Animation):
    """Lightning strike effect."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.strike_frame = 0
        self.next_strike = random.randint(50, 150)

    def update(self):
        if self.frame >= self.next_strike:
            self.strike_frame = self.frame
            self.next_strike = self.frame + random.randint(50, 150)

        age = self.frame - self.strike_frame

        if age < 2:  # Flash
            self.lcd.set_all_leds(1)
            self.lcd.set_all_colors('ffffff')
        elif age < 5:  # Afterglow
            self.lcd.set_all_leds(1)
            brightness = 0.5 - (age - 2) * 0.15
            self.lcd.set_all_colors(hsv_to_rgb(240, 0.3, brightness))
        else:
            # Dark with occasional flicker
            if random.random() < 0.05:
                self.lcd.set_all_leds(1)
                self.lcd.set_all_colors('1a1a2e')
            else:
                self.lcd.clear()

        super().update()


class Aurora(Animation):
    """Aurora borealis effect."""

    def update(self):
        self.lcd.set_all_leds(1)

        for i in range(NUMBER_OF_LEDS):
            # Layered sine waves
            v1 = math.sin(i * 0.2 + self.frame * 0.05)
            v2 = math.sin(i * 0.1 + self.frame * 0.03)
            brightness = (v1 + v2) * 0.25 + 0.5
            brightness = max(0.2, min(1.0, brightness))

            # Aurora colors: green, blue, purple
            hue = 120 + (v1 * 60)
            self.lcd.set_color(i, hsv_to_rgb(hue, 0.8, brightness))

        super().update()


class Fireflies(Animation):
    """Fireflies twinkling effect."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.fireflies = {}

    def update(self):
        self.lcd.clear()

        # Fade existing fireflies
        to_remove = []
        for led_idx, brightness in list(self.fireflies.items()):
            brightness -= 0.05
            if brightness <= 0:
                to_remove.append(led_idx)
            else:
                self.fireflies[led_idx] = brightness
                self.lcd.set_led(led_idx, 1)
                self.lcd.set_color(led_idx, hsv_to_rgb(60, 1.0, brightness))

        for idx in to_remove:
            del self.fireflies[idx]

        # Add new fireflies
        if random.random() < 0.15:
            led_idx = random.randint(0, NUMBER_OF_LEDS - 1)
            self.fireflies[led_idx] = 1.0

        super().update()


class LavaLamp(Animation):
    """Lava lamp effect."""

    def update(self):
        self.lcd.set_all_leds(1)

        for i in range(NUMBER_OF_LEDS):
            # Multiple blob simulations
            blob1 = math.sin(i * 0.3 + self.frame * 0.08) * 0.5
            blob2 = math.sin(i * 0.2 + self.frame * 0.05 + 2) * 0.5
            combined = blob1 + blob2

            brightness = (combined + 1) / 2
            # Lava colors: red-orange-yellow
            hue = 0 + (brightness * 60)

            self.lcd.set_color(i, hsv_to_rgb(hue, 1.0, 0.5 + brightness * 0.5))

        super().update()


# ============================================================================
# CLASSIC GAMES / RETRO
# ============================================================================

class Pacman(Animation):
    """Pac-man chase animation."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.pacman_pos = 0
        self.ghost_pos = -10

    def update(self):
        self.lcd.clear()

        # Move characters
        self.pacman_pos = (self.pacman_pos + 1) % (NUMBER_OF_LEDS + 20)
        self.ghost_pos = (self.ghost_pos + 1) % (NUMBER_OF_LEDS + 20)

        # Draw Pac-man (yellow)
        if 0 <= self.pacman_pos < NUMBER_OF_LEDS:
            self.lcd.set_led(self.pacman_pos, 1)
            self.lcd.set_color(self.pacman_pos, 'ffff00')

            # Pac-man mouth (one LED behind is dark)
            if (self.frame % 10) < 5:
                behind = (self.pacman_pos - 1) % NUMBER_OF_LEDS
                self.lcd.set_led(behind, 0)

        # Draw ghost (cycling colors)
        if 0 <= self.ghost_pos < NUMBER_OF_LEDS:
            ghost_color = wheel((self.frame * 5) % 256)
            self.lcd.set_led(self.ghost_pos, 1)
            self.lcd.set_color(self.ghost_pos, ghost_color)

        super().update()


class TetrisBlocks(Animation):
    """Falling Tetris blocks."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.blocks = []

    def update(self):
        self.lcd.clear()

        # Add new block occasionally
        if self.frame % 30 == 0:
            self.blocks.append({
                'pos': 0,
                'color': wheel(random.randint(0, 255)),
                'length': random.randint(3, 6)
            })

        # Update and draw blocks
        to_remove = []
        for block in self.blocks:
            block['pos'] += 0.5

            if block['pos'] > NUMBER_OF_LEDS:
                to_remove.append(block)
            else:
                for i in range(block['length']):
                    idx = int(block['pos']) + i
                    if 0 <= idx < NUMBER_OF_LEDS:
                        self.lcd.set_led(idx, 1)
                        self.lcd.set_color(idx, block['color'])

        for block in to_remove:
            self.blocks.remove(block)

        super().update()


# ============================================================================
# ADVANCED EFFECTS
# ============================================================================

class Comet(Animation):
    """Comet with long trailing tail."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.tail_length = 25

    def update(self):
        self.lcd.clear()
        head = (self.frame * 2) % NUMBER_OF_LEDS

        for i in range(self.tail_length):
            idx = (head - i) % NUMBER_OF_LEDS
            brightness = 1.0 - (i / self.tail_length)
            brightness = brightness ** 2  # Non-linear falloff

            self.lcd.set_led(idx, 1)
            # White to blue gradient for comet
            if i < 5:
                color = 'ffffff'
            else:
                color = hsv_to_rgb(200, 1.0, brightness)
            self.lcd.set_color(idx, color)

        super().update()


class Fireworks(Animation):
    """Fireworks explosion effect."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.explosions = []

    def update(self):
        self.lcd.clear()

        # Create new explosion
        if self.frame % 40 == 0:
            self.explosions.append({
                'center': random.randint(10, NUMBER_OF_LEDS - 10),
                'color': wheel(random.randint(0, 255)),
                'age': 0
            })

        # Update explosions
        to_remove = []
        for exp in self.explosions:
            exp['age'] += 1

            if exp['age'] > 30:
                to_remove.append(exp)
            else:
                radius = exp['age']
                brightness = 1.0 - (exp['age'] / 30)

                for i in range(-radius, radius + 1):
                    idx = exp['center'] + i
                    if 0 <= idx < NUMBER_OF_LEDS:
                        distance = abs(i)
                        if distance <= radius:
                            b = brightness * (1.0 - distance / radius)
                            self.lcd.set_led(idx, 1)
                            # Just use the stored wheel color with brightness
                            self.lcd.set_color(idx, exp['color'])

        for exp in to_remove:
            self.explosions.remove(exp)

        super().update()


class Waterfall(Animation):
    """Waterfall cascade effect."""

    def update(self):
        self.lcd.set_all_leds(1)

        for i in range(NUMBER_OF_LEDS):
            # Cascading wave pattern
            wave = math.sin((i - self.frame) * 0.3)
            brightness = (wave + 1) / 2

            # Water colors: cyan to blue
            hue = 180 + (brightness * 30)
            self.lcd.set_color(i, hsv_to_rgb(hue, 0.8, 0.5 + brightness * 0.5))

        super().update()


class DNAHelix(Animation):
    """DNA double helix pattern."""

    def update(self):
        self.lcd.set_all_leds(1)

        for i in range(NUMBER_OF_LEDS):
            # Two sine waves offset by 180 degrees
            strand1 = math.sin((i + self.frame) * 0.3)
            strand2 = math.sin((i + self.frame) * 0.3 + math.pi)

            # Alternate between strands
            if strand1 > strand2:
                color = hsv_to_rgb(300, 1.0, 1.0)  # Magenta
            else:
                color = hsv_to_rgb(180, 1.0, 1.0)  # Cyan

            self.lcd.set_color(i, color)

        super().update()


class RainbowSpiral(Animation):
    """Rainbow spiral effect."""

    def update(self):
        self.lcd.set_all_leds(1)

        for i in range(NUMBER_OF_LEDS):
            angle = (i / NUMBER_OF_LEDS) * math.pi * 4
            rotation = self.frame * 0.05
            hue = ((angle + rotation) * 180 / math.pi) % 360

            self.lcd.set_color(i, hsv_to_rgb(hue, 1.0, 1.0))

        super().update()


class MirrorBounce(Animation):
    """Mirror effect bouncing between sides."""

    def update(self):
        self.lcd.clear()
        pos = (self.frame // 2) % 42

        # CPU side
        if pos < len(LED_REGIONS['cpu']):
            self.lcd.set_led(LED_REGIONS['cpu'][pos], 1)
            self.lcd.set_color(LED_REGIONS['cpu'][pos], 'ff00ff')

        # GPU side (mirrored)
        mirror_pos = 41 - pos
        if mirror_pos < len(LED_REGIONS['gpu']):
            self.lcd.set_led(LED_REGIONS['gpu'][mirror_pos], 1)
            self.lcd.set_color(LED_REGIONS['gpu'][mirror_pos], '00ffff')

        super().update()


class Sunset(Animation):
    """Sunset color transition."""

    def update(self):
        self.lcd.set_all_leds(1)

        # Cycle through sunset colors
        t = (self.frame % 200) / 200.0

        if t < 0.3:  # Blue sky
            color = interpolate_color('87CEEB', 'FF6B6B', t / 0.3)
        elif t < 0.6:  # Orange
            color = interpolate_color('FF6B6B', 'FF8C00', (t - 0.3) / 0.3)
        elif t < 0.8:  # Deep orange to purple
            color = interpolate_color('FF8C00', '4A148C', (t - 0.6) / 0.2)
        else:  # Dark purple to black
            color = interpolate_color('4A148C', '0A0A0A', (t - 0.8) / 0.2)

        self.lcd.set_all_colors(color)
        super().update()


class BootSequence(Animation):
    """System boot sequence simulation."""

    def update(self):
        self.lcd.clear()

        # Fill LEDs sequentially with green
        progress = (self.frame % 100)
        num_lit = int((progress / 100.0) * NUMBER_OF_LEDS)

        for i in range(num_lit):
            self.lcd.set_led(i, 1)
            if progress > 95:  # Blink at end
                self.lcd.set_color(i, '00ff00' if (self.frame % 10) < 5 else '000000')
            else:
                self.lcd.set_color(i, '00ff00')

        super().update()


class ScanLine(Animation):
    """Scanning line effect."""

    def update(self):
        self.lcd.set_all_leds(1)
        self.lcd.set_all_colors('222222')  # Dim background

        # Moving scan line
        pos = self.frame % NUMBER_OF_LEDS

        for i in range(-3, 4):
            idx = (pos + i) % NUMBER_OF_LEDS
            brightness = 1.0 - (abs(i) / 3.0)
            self.lcd.set_led(idx, 1)
            self.lcd.set_color(idx, hsv_to_rgb(180, 1.0, brightness))

        super().update()


class Kaleidoscope(Animation):
    """Kaleidoscope pattern."""

    def update(self):
        self.lcd.set_all_leds(1)

        mid = NUMBER_OF_LEDS // 2

        for i in range(mid):
            # Create symmetric pattern
            hue = (i * 10 + self.frame * 2) % 360
            color = hsv_to_rgb(hue, 1.0, 1.0)

            self.lcd.set_color(i, color)
            self.lcd.set_color(NUMBER_OF_LEDS - 1 - i, color)

        super().update()


class TrafficLight(Animation):
    """Traffic light sequence."""

    def update(self):
        self.lcd.clear()

        # 3 second cycle: Red -> Yellow -> Green
        phase = (self.frame // 30) % 3

        if phase == 0:  # Red
            self.lcd.set_leds(LED_REGIONS['cpu'], 1)
            for i in LED_REGIONS['cpu']:
                self.lcd.set_color(i, 'ff0000')
        elif phase == 1:  # Yellow
            self.lcd.set_all_leds(1)
            self.lcd.set_all_colors('ffff00')
        else:  # Green
            self.lcd.set_leds(LED_REGIONS['gpu'], 1)
            for i in LED_REGIONS['gpu']:
                self.lcd.set_color(i, '00ff00')

        super().update()


class Meteor(Animation):
    """Meteor shower effect."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.meteors = []

    def update(self):
        self.lcd.clear()

        # Spawn new meteor
        if random.random() < 0.1:
            self.meteors.append({
                'pos': 0,
                'speed': random.uniform(1.5, 3.0),
                'color': wheel(random.randint(0, 255))
            })

        # Update meteors
        to_remove = []
        for meteor in self.meteors:
            meteor['pos'] += meteor['speed']

            if meteor['pos'] > NUMBER_OF_LEDS + 10:
                to_remove.append(meteor)
            else:
                # Draw meteor trail
                for i in range(10):
                    idx = int(meteor['pos']) - i
                    if 0 <= idx < NUMBER_OF_LEDS:
                        brightness = 1.0 - (i / 10.0)
                        self.lcd.set_led(idx, 1)
                        self.lcd.set_color(idx, hsv_to_rgb(30, 0.8, brightness))

        for meteor in to_remove:
            self.meteors.remove(meteor)

        super().update()


class RgbWindmills(Animation):
    """Rotating RGB windmill blades - many colorful windmills."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.num_mills = 12  # Maximum windmills
        self.blade_length = 3
        self.rotation_speed = 8

    def update(self):
        self.lcd.clear()

        # Create many small windmills with different colors
        for mill_idx in range(self.num_mills):
            center = int((mill_idx + 0.5) * (NUMBER_OF_LEDS / self.num_mills))
            rotation = (self.frame // self.rotation_speed + mill_idx) % 4

            # Each windmill gets a different hue
            hue = (mill_idx * 30 + self.frame) % 360
            color = hsv_to_rgb(hue, 1.0, 1.0)

            # Draw 4 blades
            for blade in range(4):
                blade_pos = (rotation + blade) % 4

                if blade_pos == 0:  # Right blade
                    for i in range(1, self.blade_length + 1):
                        led_idx = center + i
                        if 0 <= led_idx < NUMBER_OF_LEDS:
                            self.lcd.set_led(led_idx, 1)
                            self.lcd.set_color(led_idx, color)
                elif blade_pos == 1:
                    led_idx = center + 1
                    if 0 <= led_idx < NUMBER_OF_LEDS:
                        self.lcd.set_led(led_idx, 1)
                        self.lcd.set_color(led_idx, color)
                elif blade_pos == 2:  # Left blade
                    for i in range(1, self.blade_length + 1):
                        led_idx = center - i
                        if 0 <= led_idx < NUMBER_OF_LEDS:
                            self.lcd.set_led(led_idx, 1)
                            self.lcd.set_color(led_idx, color)
                elif blade_pos == 3:
                    led_idx = center - 1
                    if 0 <= led_idx < NUMBER_OF_LEDS:
                        self.lcd.set_led(led_idx, 1)
                        self.lcd.set_color(led_idx, color)

            # Center point
            if 0 <= center < NUMBER_OF_LEDS:
                self.lcd.set_led(center, 1)
                self.lcd.set_color(center, color)

        super().update()


class Bubbles(Animation):
    """Rising bubbles effect."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.bubbles = []

    def update(self):
        self.lcd.clear()

        # Create new bubbles randomly
        if random.random() < 0.2:
            self.bubbles.append({
                'pos': NUMBER_OF_LEDS - 1,
                'speed': random.uniform(0.3, 1.2),
                'color': hsv_to_rgb(random.randint(180, 240), 0.7, 1.0),
                'size': random.randint(2, 4)
            })

        # Update and draw bubbles
        to_remove = []
        for bubble in self.bubbles:
            bubble['pos'] -= bubble['speed']

            if bubble['pos'] < -bubble['size']:
                to_remove.append(bubble)
            else:
                # Draw bubble with glow
                for i in range(bubble['size']):
                    idx = int(bubble['pos']) + i
                    if 0 <= idx < NUMBER_OF_LEDS:
                        brightness = 1.0 - (abs(i - bubble['size'] / 2) / (bubble['size'] / 2))
                        self.lcd.set_led(idx, 1)
                        self.lcd.set_color(idx, bubble['color'])

        for bubble in to_remove:
            self.bubbles.remove(bubble)

        super().update()


class Stars(Animation):
    """Twinkling stars effect."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.stars = {}
        # Initialize some stars
        for _ in range(15):
            self.stars[random.randint(0, NUMBER_OF_LEDS - 1)] = {
                'brightness': random.random(),
                'speed': random.uniform(0.02, 0.08),
                'direction': random.choice([1, -1])
            }

    def update(self):
        self.lcd.clear()

        # Add new stars occasionally
        if random.random() < 0.05 and len(self.stars) < 20:
            self.stars[random.randint(0, NUMBER_OF_LEDS - 1)] = {
                'brightness': random.random(),
                'speed': random.uniform(0.02, 0.08),
                'direction': random.choice([1, -1])
            }

        # Update and draw stars
        for led_idx, star in list(self.stars.items()):
            # Twinkle
            star['brightness'] += star['speed'] * star['direction']
            if star['brightness'] >= 1.0:
                star['brightness'] = 1.0
                star['direction'] = -1
            elif star['brightness'] <= 0.1:
                star['brightness'] = 0.1
                star['direction'] = 1

            self.lcd.set_led(led_idx, 1)
            self.lcd.set_color(led_idx, hsv_to_rgb(45, 0.3, star['brightness']))

        super().update()


class Disco(Animation):
    """Disco light effect with random flashing."""

    def update(self):
        self.lcd.clear()

        # Random disco lights
        num_lights = random.randint(8, 20)
        for _ in range(num_lights):
            idx = random.randint(0, NUMBER_OF_LEDS - 1)
            hue = random.randint(0, 360)
            self.lcd.set_led(idx, 1)
            self.lcd.set_color(idx, hsv_to_rgb(hue, 1.0, 1.0))

        super().update()


class WarpSpeed(Animation):
    """Star Trek warp speed effect."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.stars = []

    def update(self):
        self.lcd.clear()

        # Create new stars at the center
        if self.frame % 2 == 0:
            self.stars.append({
                'pos': NUMBER_OF_LEDS // 2,
                'speed': 1,
                'direction': random.choice([-1, 1])
            })

        # Update and draw stars
        to_remove = []
        for star in self.stars:
            star['pos'] += star['speed'] * star['direction']
            star['speed'] *= 1.1  # Accelerate

            if star['pos'] < 0 or star['pos'] >= NUMBER_OF_LEDS:
                to_remove.append(star)
            else:
                # Draw star streak
                streak_len = min(int(star['speed']), 8)
                for i in range(streak_len):
                    idx = int(star['pos']) - i * star['direction']
                    if 0 <= idx < NUMBER_OF_LEDS:
                        brightness = 1.0 - (i / streak_len)
                        self.lcd.set_led(idx, 1)
                        self.lcd.set_color(idx, hsv_to_rgb(200, 0.5, brightness))

        for star in to_remove:
            self.stars.remove(star)

        super().update()


class BinaryRain(Animation):
    """Binary rain effect."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.columns = []
        for i in range(0, NUMBER_OF_LEDS, 4):
            self.columns.append({
                'pos': random.randint(-20, 0),
                'speed': random.uniform(0.5, 1.5),
                'bits': [random.randint(0, 1) for _ in range(10)]
            })

    def update(self):
        self.lcd.clear()

        for col in self.columns:
            col['pos'] += col['speed']

            if col['pos'] > NUMBER_OF_LEDS + 10:
                col['pos'] = -10
                col['bits'] = [random.randint(0, 1) for _ in range(10)]

            # Draw binary digits
            for i, bit in enumerate(col['bits']):
                idx = int(col['pos']) + i
                if 0 <= idx < NUMBER_OF_LEDS:
                    if bit == 1:
                        brightness = 1.0 if i == 0 else 0.5
                        self.lcd.set_led(idx, 1)
                        self.lcd.set_color(idx, hsv_to_rgb(120, 1.0, brightness))

        super().update()


class PulseRing(Animation):
    """Expanding pulse rings from center."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.rings = []

    def update(self):
        self.lcd.clear()

        # Create new ring
        if self.frame % 20 == 0:
            self.rings.append({
                'center': NUMBER_OF_LEDS // 2,
                'radius': 0,
                'color': wheel(random.randint(0, 255))
            })

        # Update and draw rings
        to_remove = []
        for ring in self.rings:
            ring['radius'] += 1.5

            if ring['radius'] > NUMBER_OF_LEDS // 2 + 5:
                to_remove.append(ring)
            else:
                # Draw ring
                for offset in [-int(ring['radius']), int(ring['radius'])]:
                    idx = ring['center'] + offset
                    if 0 <= idx < NUMBER_OF_LEDS:
                        brightness = 1.0 - (ring['radius'] / (NUMBER_OF_LEDS // 2))
                        self.lcd.set_led(idx, 1)
                        self.lcd.set_color(idx, ring['color'])

        for ring in to_remove:
            self.rings.remove(ring)

        super().update()


class ColorShift(Animation):
    """Shifting color bands with randomization."""

    def update(self):
        self.lcd.set_all_leds(1)
        band_width = random.randint(5, 15)

        for i in range(NUMBER_OF_LEDS):
            band_idx = ((i + self.frame) // band_width) % 6
            hue = (band_idx * 60 + random.randint(-10, 10)) % 360
            self.lcd.set_color(i, hsv_to_rgb(hue, 1.0, 1.0))

        super().update()


class RandomWalk(Animation):
    """Random walk particles."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.walkers = []
        for _ in range(8):
            self.walkers.append({
                'pos': random.randint(0, NUMBER_OF_LEDS - 1),
                'hue': random.randint(0, 360)
            })

    def update(self):
        self.lcd.clear()

        for walker in self.walkers:
            # Random walk
            walker['pos'] += random.choice([-2, -1, 0, 1, 2])
            walker['pos'] = walker['pos'] % NUMBER_OF_LEDS

            # Slow color shift
            walker['hue'] = (walker['hue'] + random.randint(-5, 5)) % 360

            # Draw walker with trail
            for i in range(5):
                idx = (walker['pos'] - i) % NUMBER_OF_LEDS
                brightness = 1.0 - (i / 5.0)
                self.lcd.set_led(idx, 1)
                self.lcd.set_color(idx, hsv_to_rgb(walker['hue'], 1.0, brightness))

        super().update()


class Glitch(Animation):
    """Glitch/corruption effect."""

    def update(self):
        # Random glitch intensity
        if random.random() < 0.1:
            self.lcd.clear()
            # Heavy glitch
            for _ in range(random.randint(20, 40)):
                idx = random.randint(0, NUMBER_OF_LEDS - 1)
                color = wheel(random.randint(0, 255))
                self.lcd.set_led(idx, 1)
                self.lcd.set_color(idx, color)
        else:
            # Small glitches
            for _ in range(random.randint(2, 8)):
                idx = random.randint(0, NUMBER_OF_LEDS - 1)
                if random.random() < 0.5:
                    self.lcd.set_led(idx, 0)
                else:
                    color = wheel(random.randint(0, 255))
                    self.lcd.set_led(idx, 1)
                    self.lcd.set_color(idx, color)

        super().update()


class ScannerSweep(Animation):
    """Multiple scanner beams sweeping."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.scanners = []
        for i in range(4):
            self.scanners.append({
                'pos': i * (NUMBER_OF_LEDS // 4),
                'direction': random.choice([-1, 1]),
                'hue': i * 90
            })

    def update(self):
        self.lcd.clear()

        for scanner in self.scanners:
            scanner['pos'] += scanner['direction'] * 2

            # Bounce
            if scanner['pos'] <= 0 or scanner['pos'] >= NUMBER_OF_LEDS - 1:
                scanner['direction'] *= -1
                scanner['hue'] = (scanner['hue'] + 30) % 360

            # Draw scanner beam
            for i in range(-6, 7):
                idx = scanner['pos'] + i
                if 0 <= idx < NUMBER_OF_LEDS:
                    brightness = 1.0 - (abs(i) / 6.0)
                    self.lcd.set_led(idx, 1)
                    self.lcd.set_color(idx, hsv_to_rgb(scanner['hue'], 1.0, brightness))

        super().update()


class Confetti(Animation):
    """Confetti burst effect."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.confetti = []

    def update(self):
        # Fade existing confetti
        for led_idx, (color, life) in list(self.confetti):
            life -= 1
            if life <= 0:
                self.confetti.remove((led_idx, (color, life)))
                self.lcd.set_led(led_idx, 0)
            else:
                self.confetti[self.confetti.index((led_idx, (color, life + 1)))] = (led_idx, (color, life))

        # Add new confetti bursts
        if random.random() < 0.3:
            for _ in range(random.randint(3, 8)):
                led_idx = random.randint(0, NUMBER_OF_LEDS - 1)
                color = wheel(random.randint(0, 255))
                life = random.randint(20, 40)
                self.confetti.append((led_idx, (color, life)))
                self.lcd.set_led(led_idx, 1)
                self.lcd.set_color(led_idx, color)

        super().update()


class Ripple(Animation):
    """Water ripple effect."""

    def update(self):
        self.lcd.set_all_leds(1)

        for i in range(NUMBER_OF_LEDS):
            # Multiple ripples from different centers
            v1 = math.sin((i - NUMBER_OF_LEDS // 3 - self.frame * 0.5) * 0.3)
            v2 = math.sin((i - NUMBER_OF_LEDS * 2 // 3 - self.frame * 0.5) * 0.3)
            combined = (v1 + v2) / 2

            brightness = (combined + 1) / 2
            hue = 180 + int(brightness * 60)
            self.lcd.set_color(i, hsv_to_rgb(hue, 0.8, 0.5 + brightness * 0.5))

        super().update()


# Extended animation registry
EXTENDED_ANIMATIONS = {
    'heartbeat': Heartbeat,
    'lighthouse': Lighthouse,
    'emergency_strobe': EmergencyStrobe,
    'morse_sos': MorseCodeSOS,
    'strobe_multicolor': StrobeMulticolor,
    'snake': Snake,
    'bouncing_ball': BouncingBall,
    'ping_pong': PingPong,
    'spiral': Spiral,
    'vu_meter': VUMeter,
    'equalizer': Equalizer,
    'beat_pulse': BeatPulse,
    'lightning': Lightning,
    'aurora': Aurora,
    'fireflies': Fireflies,
    'lava_lamp': LavaLamp,
    'pacman': Pacman,
    'tetris_blocks': TetrisBlocks,
    'comet': Comet,
    'fireworks': Fireworks,
    'waterfall': Waterfall,
    'dna_helix': DNAHelix,
    'rainbow_spiral': RainbowSpiral,
    'mirror_bounce': MirrorBounce,
    'sunset': Sunset,
    'boot_sequence': BootSequence,
    'scan_line': ScanLine,
    'kaleidoscope': Kaleidoscope,
    'traffic_light': TrafficLight,
    'meteor': Meteor,
    'rgb_windmills': RgbWindmills,
    'bubbles': Bubbles,
    'stars': Stars,
    'disco': Disco,
    'warp_speed': WarpSpeed,
    'binary_rain': BinaryRain,
    'pulse_ring': PulseRing,
    'color_shift': ColorShift,
    'random_walk': RandomWalk,
    'glitch': Glitch,
    'scanner_sweep': ScannerSweep,
    'confetti': Confetti,
    'ripple': Ripple,
}
