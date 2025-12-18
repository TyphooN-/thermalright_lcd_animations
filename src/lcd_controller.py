"""Base LCD controller for Thermalright USB displays."""
import hid
import numpy as np
import time
from led_map import NUMBER_OF_LEDS, LED_REGIONS


class LCDController:
    """Handles low-level communication with Thermalright LCD display."""

    def __init__(self, vendor_id=0x0416, product_id=0x8001):
        self.VENDOR_ID = vendor_id
        self.PRODUCT_ID = product_id
        self.dev = self.get_device()
        self.HEADER = 'dadbdcdd000000000000000000000000fc0000ff'
        self.leds = np.array([0] * NUMBER_OF_LEDS)
        self.colors = np.array(["ff0000"] * NUMBER_OF_LEDS)

    def get_device(self):
        """Connect to the HID device."""
        try:
            return hid.Device(self.VENDOR_ID, self.PRODUCT_ID)
        except Exception as e:
            print(f"Error initializing HID device: {e}")
            return None

    def set_led(self, index, state):
        """Set a single LED on (1) or off (0)."""
        if 0 <= index < NUMBER_OF_LEDS:
            self.leds[index] = state

    def set_leds(self, indices, state):
        """Set multiple LEDs on (1) or off (0)."""
        for idx in indices:
            if 0 <= idx < NUMBER_OF_LEDS:
                self.leds[idx] = state

    def set_color(self, index, color):
        """Set color for a single LED (hex string like 'ff0000')."""
        if 0 <= index < NUMBER_OF_LEDS:
            self.colors[index] = color

    def set_colors(self, indices, color):
        """Set color for multiple LEDs."""
        for idx in indices:
            if 0 <= idx < NUMBER_OF_LEDS:
                self.colors[idx] = color

    def set_all_leds(self, state):
        """Set all LEDs on (1) or off (0)."""
        self.leds[:] = state

    def set_all_colors(self, color):
        """Set all LED colors."""
        self.colors[:] = color

    def clear(self):
        """Turn off all LEDs."""
        self.leds[:] = 0

    def send_packets(self):
        """Send current LED state to the device."""
        if self.dev is None:
            return False

        try:
            # Build message: use color if LED is on, otherwise black
            message = "".join([
                self.colors[i] if self.leds[i] != 0 else "000000"
                for i in range(NUMBER_OF_LEDS)
            ])

            # Send header packet
            packet0 = bytes.fromhex(self.HEADER + message[:128-len(self.HEADER)])
            self.dev.write(packet0)

            # Send remaining data packets
            packets = message[88:]
            for i in range(0, 4):
                packet = bytes.fromhex('00' + packets[i*128:(i+1)*128])
                self.dev.write(packet)

            return True
        except Exception as e:
            print(f"Error sending packets: {e}")
            return False

    def close(self):
        """Close the device connection."""
        if self.dev:
            try:
                self.dev.close()
            except:
                pass
