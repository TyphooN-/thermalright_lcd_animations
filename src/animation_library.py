"""Animation library with various LED animation effects."""
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
# WAVE ANIMATIONS
# ============================================================================

class RainbowWaveLTR(Animation):
    """Rainbow wave moving left to right."""

    def update(self):
        self.lcd.set_all_leds(1)
        for i in range(NUMBER_OF_LEDS):
            hue = (self.frame * 3 + i * 4) % 360
            self.lcd.set_color(i, hsv_to_rgb(hue, 1.0, 1.0))
        super().update()


class RainbowWaveRTL(Animation):
    """Rainbow wave moving right to left."""

    def update(self):
        self.lcd.set_all_leds(1)
        for i in range(NUMBER_OF_LEDS):
            hue = (self.frame * 3 - i * 4) % 360
            self.lcd.set_color(i, hsv_to_rgb(hue, 1.0, 1.0))
        super().update()


class DualWave(Animation):
    """Dual rainbow waves on CPU and GPU sides."""

    def update(self):
        self.lcd.set_all_leds(1)
        # CPU side
        for i in LED_REGIONS['cpu']:
            hue = (self.frame * 5 + i * 8) % 360
            self.lcd.set_color(i, hsv_to_rgb(hue, 1.0, 1.0))
        # GPU side
        for i in LED_REGIONS['gpu']:
            hue = (self.frame * 5 - i * 8) % 360
            self.lcd.set_color(i, hsv_to_rgb(hue, 1.0, 1.0))
        super().update()


class OceanWave(Animation):
    """Ocean-themed wave animation."""

    def update(self):
        self.lcd.set_all_leds(1)
        palette = get_palette('ocean')
        for i in range(NUMBER_OF_LEDS):
            offset = math.sin((self.frame + i * 3) * 0.1) * 0.5 + 0.5
            idx = int(offset * (len(palette) - 1))
            self.lcd.set_color(i, palette[idx])
        super().update()


class FireWave(Animation):
    """Fire-themed wave animation."""

    def update(self):
        self.lcd.set_all_leds(1)
        palette = get_palette('fire')
        for i in range(NUMBER_OF_LEDS):
            # Increased flickering effect with more randomness
            offset = abs(math.sin((self.frame * 0.15 + i * 0.1)) + random.uniform(-0.4, 0.4))
            offset = max(0, min(1, offset))
            # Add extra random sparks
            if random.random() < 0.1:
                offset = random.uniform(0.8, 1.0)
            idx = int(offset * (len(palette) - 1))
            self.lcd.set_color(i, palette[idx])
        super().update()


# ============================================================================
# SCANNER/CHASE ANIMATIONS
# ============================================================================

class KnightRider(Animation):
    """Knight Rider / K.I.T.T. scanner effect."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.tail_length = 8
        self.direction = 1

    def update(self):
        self.lcd.clear()
        pos = (self.frame // 2) % (NUMBER_OF_LEDS * 2)

        if pos >= NUMBER_OF_LEDS:
            pos = NUMBER_OF_LEDS * 2 - pos - 1

        for i in range(self.tail_length):
            led_idx = pos - i
            if 0 <= led_idx < NUMBER_OF_LEDS:
                brightness = 1.0 - (i / self.tail_length)
                color = hsv_to_rgb(0, 1.0, brightness)  # Red
                self.lcd.set_led(led_idx, 1)
                self.lcd.set_color(led_idx, color)
        super().update()


class CylonEye(Animation):
    """Cylon eye scanner (both sides)."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.tail_length = 6

    def update(self):
        self.lcd.clear()
        pos = (self.frame // 2) % (42 * 2)

        for side in ['cpu', 'gpu']:
            side_pos = pos if pos < 42 else 84 - pos - 1
            leds = LED_REGIONS[side]

            for i in range(self.tail_length):
                idx_in_side = side_pos - i
                if 0 <= idx_in_side < len(leds):
                    led_idx = leds[idx_in_side]
                    brightness = 1.0 - (i / self.tail_length)
                    color = hsv_to_rgb(0, 1.0, brightness)
                    self.lcd.set_led(led_idx, 1)
                    self.lcd.set_color(led_idx, color)
        super().update()


class LarsonScannerDual(Animation):
    """Dual Larson scanners on CPU and GPU."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.tail_length = 10

    def update(self):
        self.lcd.clear()
        cpu_pos = (self.frame // 2) % (42 * 2)
        gpu_pos = (NUMBER_OF_LEDS - (self.frame // 2)) % (42 * 2)

        if cpu_pos >= 42:
            cpu_pos = 84 - cpu_pos - 1
        if gpu_pos >= 42:
            gpu_pos = 84 - gpu_pos - 1

        # CPU scanner (cyan)
        for i in range(self.tail_length):
            idx = cpu_pos - i
            if 0 <= idx < 42:
                brightness = 1.0 - (i / self.tail_length)
                self.lcd.set_led(idx, 1)
                self.lcd.set_color(idx, hsv_to_rgb(180, 1.0, brightness))

        # GPU scanner (magenta)
        for i in range(self.tail_length):
            idx = 42 + gpu_pos - i
            if 42 <= idx < NUMBER_OF_LEDS:
                brightness = 1.0 - (i / self.tail_length)
                self.lcd.set_led(idx, 1)
                self.lcd.set_color(idx, hsv_to_rgb(300, 1.0, brightness))

        super().update()


class ChasingLights(Animation):
    """Chasing lights effect."""

    def update(self):
        self.lcd.clear()
        spacing = 8
        for i in range(NUMBER_OF_LEDS):
            if (i + self.frame) % spacing == 0:
                hue = ((i // spacing) * 60) % 360
                self.lcd.set_led(i, 1)
                self.lcd.set_color(i, hsv_to_rgb(hue, 1.0, 1.0))
        super().update()


class TheaterChase(Animation):
    """Theater marquee chase effect."""

    def update(self):
        self.lcd.clear()
        q = self.frame % 3
        for i in range(0, NUMBER_OF_LEDS, 3):
            idx = i + q
            if idx < NUMBER_OF_LEDS:
                hue = (self.frame * 2) % 360
                self.lcd.set_led(idx, 1)
                self.lcd.set_color(idx, hsv_to_rgb(hue, 1.0, 1.0))
        super().update()


# ============================================================================
# PATTERN ANIMATIONS
# ============================================================================

class PoliceStrobe(Animation):
    """Police strobe light effect."""

    def update(self):
        self.lcd.clear()
        cycle = (self.frame // 5) % 4

        if cycle < 2:  # Red on left
            self.lcd.set_leds(LED_REGIONS['cpu'], 1)
            self.lcd.set_all_colors('ff0000')
        else:  # Blue on right
            self.lcd.set_leds(LED_REGIONS['gpu'], 1)
            for i in LED_REGIONS['gpu']:
                self.lcd.set_color(i, '0000ff')

        super().update()


class Checkerboard(Animation):
    """Checkerboard pattern."""

    def update(self):
        self.lcd.clear()
        offset = (self.frame // 10) % 2
        color1 = wheel((self.frame * 2) % 256)
        color2 = wheel(((self.frame * 2) + 128) % 256)

        for i in range(NUMBER_OF_LEDS):
            if (i + offset) % 2 == 0:
                self.lcd.set_led(i, 1)
                self.lcd.set_color(i, color1)
            else:
                self.lcd.set_led(i, 1)
                self.lcd.set_color(i, color2)

        super().update()


class AlternatingBars(Animation):
    """Alternating color bars."""

    def update(self):
        self.lcd.set_all_leds(1)
        bar_width = 7
        num_bars = NUMBER_OF_LEDS // bar_width + 1

        for i in range(NUMBER_OF_LEDS):
            bar_idx = (i // bar_width + (self.frame // 10)) % num_bars
            hue = (bar_idx * 60) % 360
            self.lcd.set_color(i, hsv_to_rgb(hue, 1.0, 1.0))

        super().update()


# ============================================================================
# EFFECT ANIMATIONS
# ============================================================================

class ColorBreathing(Animation):
    """Smooth breathing effect with color cycle."""

    def update(self):
        self.lcd.set_all_leds(1)
        brightness = (math.sin(self.frame * 0.05) + 1) / 2
        hue = (self.frame * 0.5) % 360

        color = hsv_to_rgb(hue, 1.0, brightness)
        self.lcd.set_all_colors(color)

        super().update()


class RainbowCycle(Animation):
    """Rainbow color cycle all LEDs same color."""

    def update(self):
        self.lcd.set_all_leds(1)
        hue = (self.frame * 2) % 360
        color = hsv_to_rgb(hue, 1.0, 1.0)
        self.lcd.set_all_colors(color)
        super().update()


class Sparkle(Animation):
    """Random sparkle/twinkle effect."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.active_leds = {}

    def update(self):
        # Fade existing sparkles
        to_remove = []
        for led_idx, (color, life) in self.active_leds.items():
            life -= 1
            if life <= 0:
                to_remove.append(led_idx)
                self.lcd.set_led(led_idx, 0)
            else:
                self.active_leds[led_idx] = (color, life)

        for idx in to_remove:
            del self.active_leds[idx]

        # Add new sparkles with higher frequency and color variety
        num_new = random.randint(1, 3) if random.random() < 0.4 else 0
        for _ in range(num_new):
            led_idx = random.randint(0, NUMBER_OF_LEDS - 1)
            # More color variety
            color = hsv_to_rgb(random.randint(0, 360), random.uniform(0.8, 1.0), 1.0)
            life = random.randint(10, 35)
            self.active_leds[led_idx] = (color, life)
            self.lcd.set_led(led_idx, 1)
            self.lcd.set_color(led_idx, color)

        super().update()


class RandomBurst(Animation):
    """Random color bursts with varied patterns."""

    def update(self):
        # Varied timing
        if self.frame % random.randint(10, 20) == 0:
            self.lcd.clear()
            num_leds = random.randint(8, 25)

            # Sometimes use single color, sometimes use multiple
            if random.random() < 0.6:
                color = hsv_to_rgb(random.randint(0, 360), 1.0, 1.0)
                for _ in range(num_leds):
                    idx = random.randint(0, NUMBER_OF_LEDS - 1)
                    self.lcd.set_led(idx, 1)
                    self.lcd.set_color(idx, color)
            else:
                # Multi-color burst
                for _ in range(num_leds):
                    idx = random.randint(0, NUMBER_OF_LEDS - 1)
                    color = hsv_to_rgb(random.randint(0, 360), 1.0, 1.0)
                    self.lcd.set_led(idx, 1)
                    self.lcd.set_color(idx, color)

        super().update()


class GradientSweep(Animation):
    """Gradient sweep across display."""

    def update(self):
        self.lcd.set_all_leds(1)
        gradient = get_gradient(['ff0000', 'ffff00', '00ff00', '00ffff', '0000ff', 'ff00ff'], NUMBER_OF_LEDS)

        for i in range(NUMBER_OF_LEDS):
            idx = (i + self.frame) % NUMBER_OF_LEDS
            self.lcd.set_color(i, gradient[idx])

        super().update()


class Plasma(Animation):
    """Plasma effect with randomized parameters."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.offset = random.uniform(0, 100)
        self.speed_mult = random.uniform(0.8, 1.2)

    def update(self):
        self.lcd.set_all_leds(1)

        # Occasionally shift parameters for variety
        if self.frame % 200 == 0:
            self.offset = random.uniform(0, 100)
            self.speed_mult = random.uniform(0.8, 1.2)

        for i in range(NUMBER_OF_LEDS):
            # Plasma formula with randomized parameters
            v = math.sin(i * 0.3 + self.frame * 0.1 * self.speed_mult + self.offset)
            v += math.sin((i * 0.2 + self.frame * 0.15 * self.speed_mult) * 1.5)
            v += math.sin(math.sqrt((i * 0.1)**2 + (self.frame * 0.08 * self.speed_mult)**2))
            v = (v + 3) / 6  # Normalize to 0-1

            hue = (int(v * 360) + int(self.offset * 3)) % 360
            self.lcd.set_color(i, hsv_to_rgb(hue, 1.0, 1.0))

        super().update()


class MatrixRain(Animation):
    """Matrix-style rain effect with randomized colors."""

    def __init__(self, lcd):
        super().__init__(lcd)
        self.drops = []
        for _ in range(12):
            self.drops.append({
                'position': random.randint(0, NUMBER_OF_LEDS - 1),
                'speed': random.uniform(0.5, 2.5),
                'length': random.randint(5, 15),
                'hue': random.randint(100, 140)  # Green variations
            })

    def update(self):
        self.lcd.clear()

        for drop in self.drops:
            drop['position'] += drop['speed']
            if drop['position'] > NUMBER_OF_LEDS + drop['length']:
                drop['position'] = -drop['length']
                drop['speed'] = random.uniform(0.5, 2.5)
                drop['length'] = random.randint(5, 15)
                drop['hue'] = random.randint(100, 140)

            for i in range(drop['length']):
                idx = int(drop['position'] - i)
                if 0 <= idx < NUMBER_OF_LEDS:
                    brightness = 1.0 - (i / drop['length'])
                    # Add slight hue variation
                    hue = drop['hue'] + random.randint(-10, 10)
                    self.lcd.set_led(idx, 1)
                    self.lcd.set_color(idx, hsv_to_rgb(hue, 1.0, brightness))

        super().update()


# ============================================================================
# DISPLAY ANIMATIONS
# ============================================================================

class BinaryCounter(Animation):
    """Binary counter animation."""

    def update(self):
        self.lcd.clear()
        count = (self.frame // 10) % 256

        for i in range(min(8, NUMBER_OF_LEDS)):
            if count & (1 << i):
                self.lcd.set_led(i, 1)
                hue = (i * 45) % 360
                self.lcd.set_color(i, hsv_to_rgb(hue, 1.0, 1.0))

        super().update()


class SegmentCrawl(Animation):
    """Crawl through display segments."""

    def update(self):
        self.lcd.clear()
        idx = (self.frame // 3) % NUMBER_OF_LEDS
        tail_length = 15

        for i in range(tail_length):
            led_idx = (idx - i) % NUMBER_OF_LEDS
            brightness = 1.0 - (i / tail_length)
            hue = (self.frame + i * 5) % 360
            self.lcd.set_led(led_idx, 1)
            self.lcd.set_color(led_idx, hsv_to_rgb(hue, 1.0, brightness))

        super().update()


class LoadingBar(Animation):
    """Loading bar animation."""

    def update(self):
        self.lcd.clear()
        progress = (self.frame % 100) / 100.0
        num_lit = int(progress * NUMBER_OF_LEDS)

        for i in range(num_lit):
            factor = i / NUMBER_OF_LEDS
            color = interpolate_color('00ff00', 'ff0000', factor)
            self.lcd.set_led(i, 1)
            self.lcd.set_color(i, color)

        super().update()


class ColorWipe(Animation):
    """Color wipe effect."""

    def update(self):
        cycle_length = NUMBER_OF_LEDS + 20
        pos = self.frame % cycle_length

        if pos < NUMBER_OF_LEDS:
            hue = (self.frame // cycle_length * 60) % 360
            color = hsv_to_rgb(hue, 1.0, 1.0)
            self.lcd.set_led(pos, 1)
            self.lcd.set_color(pos, color)
        else:
            # Clear phase
            clear_pos = pos - NUMBER_OF_LEDS
            if clear_pos < NUMBER_OF_LEDS:
                self.lcd.set_led(clear_pos, 0)

        super().update()


class RainbowSegments(Animation):
    """Each segment region gets different rainbow color."""

    def update(self):
        self.lcd.set_all_leds(1)

        regions = [
            LED_REGIONS['cpu_led'],
            LED_REGIONS['cpu_temp'],
            LED_REGIONS['cpu_celsius'] + LED_REGIONS['cpu_fahrenheit'],
            LED_REGIONS['cpu_usage_1_indicators'],
            LED_REGIONS['cpu_usage'],
            LED_REGIONS['cpu_percent_led'],
            LED_REGIONS['gpu_percent_led'],
            LED_REGIONS['gpu_usage'],
            LED_REGIONS['gpu_usage_1_indicators'],
            LED_REGIONS['gpu_celsius'] + LED_REGIONS['gpu_fahrenheit'],
            LED_REGIONS['gpu_temp'],
            LED_REGIONS['gpu_led'],
        ]

        for region_idx, region in enumerate(regions):
            hue = ((self.frame * 2) + region_idx * 30) % 360
            color = hsv_to_rgb(hue, 1.0, 1.0)
            self.lcd.set_colors(region, color)

        super().update()


# Animation registry
ANIMATIONS = {
    'rainbow_wave_ltr': RainbowWaveLTR,
    'rainbow_wave_rtl': RainbowWaveRTL,
    'dual_wave': DualWave,
    'ocean_wave': OceanWave,
    'fire_wave': FireWave,
    'knight_rider': KnightRider,
    'cylon_eye': CylonEye,
    'larson_scanner_dual': LarsonScannerDual,
    'chasing_lights': ChasingLights,
    'theater_chase': TheaterChase,
    'police_strobe': PoliceStrobe,
    'checkerboard': Checkerboard,
    'alternating_bars': AlternatingBars,
    'color_breathing': ColorBreathing,
    'rainbow_cycle': RainbowCycle,
    'sparkle': Sparkle,
    'random_burst': RandomBurst,
    'gradient_sweep': GradientSweep,
    'plasma': Plasma,
    'matrix_rain': MatrixRain,
    'binary_counter': BinaryCounter,
    'segment_crawl': SegmentCrawl,
    'loading_bar': LoadingBar,
    'color_wipe': ColorWipe,
    'rainbow_segments': RainbowSegments,
}

# Import extended animations
try:
    from animation_library_extended import EXTENDED_ANIMATIONS
    ANIMATIONS.update(EXTENDED_ANIMATIONS)
except ImportError:
    pass  # Extended animations not available
