"""Color utility functions for animations."""
import random
import math


def hex_to_rgb(hex_color):
    """Convert hex color string to RGB tuple."""
    hex_color = hex_color.lstrip('#')
    return tuple(int(hex_color[i:i+2], 16) for i in (0, 2, 4))


def rgb_to_hex(rgb):
    """Convert RGB tuple to hex color string."""
    return '{:02x}{:02x}{:02x}'.format(
        int(rgb[0]), int(rgb[1]), int(rgb[2])
    )


def interpolate_color(color1, color2, factor):
    """Interpolate between two hex colors by factor (0.0 to 1.0)."""
    rgb1 = hex_to_rgb(color1)
    rgb2 = hex_to_rgb(color2)

    r = rgb1[0] + (rgb2[0] - rgb1[0]) * factor
    g = rgb1[1] + (rgb2[1] - rgb1[1]) * factor
    b = rgb1[2] + (rgb2[2] - rgb1[2]) * factor

    return rgb_to_hex((r, g, b))


def hsv_to_rgb(h, s, v):
    """Convert HSV to RGB. H: 0-360, S: 0-1, V: 0-1"""
    h = h % 360
    c = v * s
    x = c * (1 - abs((h / 60) % 2 - 1))
    m = v - c

    if 0 <= h < 60:
        r, g, b = c, x, 0
    elif 60 <= h < 120:
        r, g, b = x, c, 0
    elif 120 <= h < 180:
        r, g, b = 0, c, x
    elif 180 <= h < 240:
        r, g, b = 0, x, c
    elif 240 <= h < 300:
        r, g, b = x, 0, c
    else:
        r, g, b = c, 0, x

    return rgb_to_hex((
        (r + m) * 255,
        (g + m) * 255,
        (b + m) * 255
    ))


def wheel(pos):
    """Generate rainbow colors across 0-255 positions."""
    pos = pos % 256
    if pos < 85:
        return rgb_to_hex((pos * 3, 255 - pos * 3, 0))
    elif pos < 170:
        pos -= 85
        return rgb_to_hex((255 - pos * 3, 0, pos * 3))
    else:
        pos -= 170
        return rgb_to_hex((0, pos * 3, 255 - pos * 3))


def random_color():
    """Generate a random color."""
    return rgb_to_hex((
        random.randint(0, 255),
        random.randint(0, 255),
        random.randint(0, 255)
    ))


def get_gradient(colors, steps):
    """Generate a gradient between multiple colors."""
    if len(colors) < 2:
        return [colors[0]] * steps

    gradient = []
    segment_steps = steps // (len(colors) - 1)

    for i in range(len(colors) - 1):
        for j in range(segment_steps):
            factor = j / segment_steps
            gradient.append(interpolate_color(colors[i], colors[i + 1], factor))

    # Fill remaining steps
    while len(gradient) < steps:
        gradient.append(colors[-1])

    return gradient[:steps]


# Predefined color palettes
PALETTES = {
    'rainbow': ['ff0000', 'ff7f00', 'ffff00', '00ff00', '0000ff', '4b0082', '9400d3'],
    'fire': ['ff0000', 'ff4500', 'ff8c00', 'ffd700', 'ffff00'],
    'ocean': ['000080', '0000ff', '00bfff', '00ffff', '40e0d0'],
    'sunset': ['ff0000', 'ff4500', 'ff6347', 'ff7f50', 'ffa500', 'ffd700'],
    'forest': ['006400', '228b22', '32cd32', '00ff00', '7fff00'],
    'cool': ['0000ff', '00ffff', '00ff00', 'ffff00'],
    'warm': ['ff0000', 'ff4500', 'ff8c00', 'ffff00'],
    'neon': ['ff00ff', '00ffff', '00ff00', 'ffff00', 'ff0000'],
    'purple_pink': ['4b0082', '8b00ff', 'ff00ff', 'ff69b4'],
    'ice': ['e0ffff', 'afeeee', '87ceeb', '4682b4', '0000ff'],
}


def get_palette(name):
    """Get a color palette by name."""
    return PALETTES.get(name, PALETTES['rainbow'])
