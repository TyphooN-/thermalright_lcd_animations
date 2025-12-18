"""LED layout mappings for Thermalright Peerless Assassin displays."""

NUMBER_OF_LEDS = 84

# LED layout mappings for Peerless Assassin 120/140 Digital
# Based on actual hardware layout
LED_REGIONS = {
    'all': list(range(0, 84)),
    'cpu': list(range(0, 42)),
    'gpu': list(range(42, 84)),
    'cpu_led': [0, 1],
    'cpu_temp': list(range(2, 23)),  # LEDs 2-22 (21 LEDs for 3 digits)
    'cpu_celsius': [23],
    'cpu_fahrenheit': [24],
    'cpu_usage_1_indicators': [25, 26],  # Top/bottom "1" indicators
    'cpu_usage': list(range(27, 41)),  # LEDs 27-40 (14 LEDs for 2 digits)
    'cpu_percent_led': [41],
    'gpu_percent_led': [42],
    'gpu_usage': list(range(43, 57)),  # LEDs 43-56 (14 LEDs for 2 digits)
    'gpu_usage_1_indicators': [57, 58],  # Top/bottom "1" indicators
    'gpu_celsius': [59],
    'gpu_fahrenheit': [60],
    'gpu_temp': list(range(61, 82)),  # LEDs 61-81 (21 LEDs for 3 digits)
    'gpu_led': [82, 83],
}
