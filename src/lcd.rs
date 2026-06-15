#![allow(dead_code)]

use crate::color::Rgb;
use crate::led_map::NUMBER_OF_LEDS;
use hidapi::{HidApi, HidDevice};

const HEADER: [u8; 20] = [
    0xda, 0xdb, 0xdc, 0xdd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0xfc, 0x00, 0x00, 0xff,
];

pub struct LcdController {
    api: HidApi,
    vendor_id: u16,
    product_id: u16,
    device: Option<HidDevice>,
    leds: [bool; NUMBER_OF_LEDS],
    colors: [Rgb; NUMBER_OF_LEDS],
}

impl LcdController {
    pub fn new(vendor_id: u16, product_id: u16) -> Result<Self, hidapi::HidError> {
        let api = HidApi::new()?;
        let device = api.open(vendor_id, product_id).ok();
        Ok(Self {
            api,
            vendor_id,
            product_id,
            device,
            leds: [true; NUMBER_OF_LEDS],
            colors: [[0xff, 0xff, 0xff]; NUMBER_OF_LEDS],
        })
    }

    pub fn is_connected(&self) -> bool {
        self.device.is_some()
    }

    #[inline]
    pub fn set_led(&mut self, index: usize, state: bool) {
        if index < NUMBER_OF_LEDS {
            self.leds[index] = state;
        }
    }

    #[inline]
    pub fn set_leds(&mut self, indices: &[usize], state: bool) {
        for &i in indices {
            if i < NUMBER_OF_LEDS {
                self.leds[i] = state;
            }
        }
    }

    #[inline]
    pub fn set_color(&mut self, index: usize, rgb: Rgb) {
        if index < NUMBER_OF_LEDS {
            self.colors[index] = rgb;
        }
    }

    #[inline]
    pub fn set_colors(&mut self, indices: &[usize], rgb: Rgb) {
        for &i in indices {
            if i < NUMBER_OF_LEDS {
                self.colors[i] = rgb;
            }
        }
    }

    #[inline]
    pub fn set_all_leds(&mut self, state: bool) {
        self.leds = [state; NUMBER_OF_LEDS];
    }

    #[inline]
    pub fn set_all_colors(&mut self, rgb: Rgb) {
        self.colors = [rgb; NUMBER_OF_LEDS];
    }

    #[inline]
    pub fn clear(&mut self) {
        self.leds = [false; NUMBER_OF_LEDS];
    }

    pub fn send_packets(&mut self) -> bool {
        let Some(dev) = self.device.as_ref() else {
            return false;
        };

        let mut color_data = [0u8; NUMBER_OF_LEDS * 3];
        for i in 0..NUMBER_OF_LEDS {
            if self.leds[i] {
                color_data[i * 3] = self.colors[i][0];
                color_data[i * 3 + 1] = self.colors[i][1];
                color_data[i * 3 + 2] = self.colors[i][2];
            }
        }

        let mut packet0 = [0u8; 64];
        packet0[..20].copy_from_slice(&HEADER);
        packet0[20..].copy_from_slice(&color_data[..44]);

        if dev.write(&packet0).is_err() {
            return false;
        }

        let remaining = &color_data[44..];
        let mut buf = [0u8; 65];
        for i in 0..3 {
            buf[0] = 0x00;
            buf[1..].copy_from_slice(&remaining[i * 64..(i + 1) * 64]);
            if dev.write(&buf).is_err() {
                return false;
            }
        }

        let mut last = [0u8; 17];
        last[0] = 0x00;
        last[1..].copy_from_slice(&remaining[192..208]);
        if dev.write(&last).is_err() {
            return false;
        }

        true
    }

    pub fn try_reconnect(&mut self) -> bool {
        if self.device.is_some() {
            return true;
        }
        self.device = self.api.open(self.vendor_id, self.product_id).ok();
        self.device.is_some()
    }

    pub fn close(&mut self) {
        self.device = None;
    }
}
