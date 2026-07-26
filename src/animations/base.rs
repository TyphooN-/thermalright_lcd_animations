use std::collections::HashMap;

use rand::RngExt;

use super::Animation;
use crate::color::{self, palette, Rgb};
use crate::lcd::LcdController;
use crate::led_map::{
    CPU_ALL, CPU_CELSIUS, CPU_FAHRENHEIT, CPU_LED, CPU_PERCENT_LED, CPU_TEMP, CPU_USAGE,
    CPU_USAGE_1_INDICATORS, GPU_ALL, GPU_CELSIUS, GPU_FAHRENHEIT, GPU_LED, GPU_PERCENT_LED,
    GPU_TEMP, GPU_USAGE, GPU_USAGE_1_INDICATORS, NUMBER_OF_LEDS,
};

// ---------------------------------------------------------------------------
// Waves
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct RainbowWaveLtr {
    frame: u32,
}
impl Animation for RainbowWaveLtr {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        for i in 0..NUMBER_OF_LEDS {
            let hue = ((self.frame as i64 * 3 + i as i64 * 4).rem_euclid(360)) as f32;
            lcd.set_color(i, color::hsv(hue, 1.0, 1.0));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct RainbowWaveRtl {
    frame: u32,
}
impl Animation for RainbowWaveRtl {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        for i in 0..NUMBER_OF_LEDS {
            let hue = ((self.frame as i64 * 3 - i as i64 * 4).rem_euclid(360)) as f32;
            lcd.set_color(i, color::hsv(hue, 1.0, 1.0));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct DualWave {
    frame: u32,
}
impl Animation for DualWave {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        for &i in CPU_ALL {
            let hue = ((self.frame as i64 * 5 + i as i64 * 8).rem_euclid(360)) as f32;
            lcd.set_color(i, color::hsv(hue, 1.0, 1.0));
        }
        for &i in GPU_ALL {
            let hue = ((self.frame as i64 * 5 - i as i64 * 8).rem_euclid(360)) as f32;
            lcd.set_color(i, color::hsv(hue, 1.0, 1.0));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct OceanWave {
    frame: u32,
}
impl Animation for OceanWave {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let pal = palette::OCEAN;
        for i in 0..NUMBER_OF_LEDS {
            let offset = ((self.frame as f32 + i as f32 * 3.0) * 0.1).sin() * 0.5 + 0.5;
            let idx = (offset * (pal.len() - 1) as f32) as usize;
            lcd.set_color(i, pal[idx.min(pal.len() - 1)]);
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct FireWave {
    frame: u32,
}
impl Animation for FireWave {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let pal = palette::FIRE;
        let mut rng = rand::rng();
        let t = self.frame as f32 * 0.09;
        for i in 0..NUMBER_OF_LEDS {
            let x = i as f32;
            let flame = (x * 0.18 + t).sin() * 0.45
                + (x * 0.41 - t * 1.6).sin() * 0.25
                + rng.random_range(-0.08..0.08);
            let ember = if rng.random::<f32>() < 0.025 {
                rng.random_range(0.15..0.35)
            } else {
                0.0
            };
            let offset = (flame.abs() + ember).clamp(0.0, 1.0);
            let idx = (offset * (pal.len() - 1) as f32) as usize;
            let mut c = pal[idx.min(pal.len() - 1)];
            let glow = 0.65 + offset * 0.35;
            c = color::scale(c, glow);
            lcd.set_color(i, c);
        }
        self.frame = self.frame.wrapping_add(1);
    }

    fn preferred_duration(&self) -> f32 {
        12.0
    }
}

// ---------------------------------------------------------------------------
// Scanners / chase
// ---------------------------------------------------------------------------

pub struct KnightRider {
    frame: u32,
    tail: usize,
}
impl Default for KnightRider {
    fn default() -> Self {
        Self { frame: 0, tail: 8 }
    }
}
impl Animation for KnightRider {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut pos = (self.frame / 2) as i32 % (NUMBER_OF_LEDS as i32 * 2);
        if pos >= NUMBER_OF_LEDS as i32 {
            pos = NUMBER_OF_LEDS as i32 * 2 - pos - 1;
        }
        for i in 0..self.tail {
            let idx = pos - i as i32;
            if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                let b = 1.0 - (i as f32 / self.tail as f32);
                lcd.set_led(idx as usize, true);
                lcd.set_color(idx as usize, color::hsv(0.0, 1.0, b));
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

pub struct CylonEye {
    frame: u32,
    tail: usize,
}
impl Default for CylonEye {
    fn default() -> Self {
        Self { frame: 0, tail: 6 }
    }
}
impl Animation for CylonEye {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let pos = (self.frame / 2) as i32 % 84;
        let side_pos = if pos < 42 { pos } else { 84 - pos - 1 };
        for &sides in &[CPU_ALL, GPU_ALL] {
            for i in 0..self.tail {
                let idx_in_side = side_pos - i as i32;
                if idx_in_side >= 0 && (idx_in_side as usize) < sides.len() {
                    let led = sides[idx_in_side as usize];
                    let b = 1.0 - (i as f32 / self.tail as f32);
                    lcd.set_led(led, true);
                    lcd.set_color(led, color::hsv(0.0, 1.0, b));
                }
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

pub struct LarsonScannerDual {
    frame: u32,
    tail: usize,
}
impl Default for LarsonScannerDual {
    fn default() -> Self {
        Self { frame: 0, tail: 10 }
    }
}
impl Animation for LarsonScannerDual {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut cpu_pos = (self.frame / 2) as i32 % 84;
        let mut gpu_pos = (NUMBER_OF_LEDS as i32 - (self.frame / 2) as i32).rem_euclid(84);
        if cpu_pos >= 42 {
            cpu_pos = 84 - cpu_pos - 1;
        }
        if gpu_pos >= 42 {
            gpu_pos = 84 - gpu_pos - 1;
        }
        for i in 0..self.tail {
            let idx = cpu_pos - i as i32;
            if (0..42).contains(&idx) {
                let b = 1.0 - (i as f32 / self.tail as f32);
                lcd.set_led(idx as usize, true);
                lcd.set_color(idx as usize, color::hsv(180.0, 1.0, b));
            }
        }
        for i in 0..self.tail {
            let idx = 42 + gpu_pos - i as i32;
            if (42..NUMBER_OF_LEDS as i32).contains(&idx) {
                let b = 1.0 - (i as f32 / self.tail as f32);
                lcd.set_led(idx as usize, true);
                lcd.set_color(idx as usize, color::hsv(300.0, 1.0, b));
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct ChasingLights {
    frame: u32,
}
impl Animation for ChasingLights {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let spacing: usize = 8;
        for i in 0..NUMBER_OF_LEDS {
            if (i + self.frame as usize) % spacing == 0 {
                let hue = ((i / spacing) as i32 * 60).rem_euclid(360) as f32;
                lcd.set_led(i, true);
                lcd.set_color(i, color::hsv(hue, 1.0, 1.0));
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct TheaterChase {
    frame: u32,
}
impl Animation for TheaterChase {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let q = (self.frame % 3) as usize;
        let mut i = 0;
        while i < NUMBER_OF_LEDS {
            let idx = i + q;
            if idx < NUMBER_OF_LEDS {
                let hue = ((self.frame as i64 * 2).rem_euclid(360)) as f32;
                lcd.set_led(idx, true);
                lcd.set_color(idx, color::hsv(hue, 1.0, 1.0));
            }
            i += 3;
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Patterns
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct Checkerboard {
    frame: u32,
}
impl Animation for Checkerboard {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let offset = ((self.frame / 10) % 2) as usize;
        let c1 = color::wheel(self.frame * 2);
        let c2 = color::wheel(self.frame * 2 + 128);
        for i in 0..NUMBER_OF_LEDS {
            lcd.set_led(i, true);
            lcd.set_color(i, if (i + offset) % 2 == 0 { c1 } else { c2 });
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct AlternatingBars {
    frame: u32,
}
impl Animation for AlternatingBars {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let bar_w: usize = 7;
        let nbars = NUMBER_OF_LEDS / bar_w + 1;
        for i in 0..NUMBER_OF_LEDS {
            let bar_idx = (i / bar_w + (self.frame / 10) as usize) % nbars;
            let hue = ((bar_idx as i32 * 60).rem_euclid(360)) as f32;
            lcd.set_color(i, color::hsv(hue, 1.0, 1.0));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Effects
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct ColorBreathing {
    frame: u32,
}
impl Animation for ColorBreathing {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let brightness = ((self.frame as f32 * 0.05).sin() + 1.0) / 2.0;
        let hue = (self.frame as f32 * 0.5) % 360.0;
        lcd.set_all_colors(color::hsv(hue, 1.0, brightness));
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct RainbowCycle {
    frame: u32,
}
impl Animation for RainbowCycle {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let hue = ((self.frame as i64 * 2).rem_euclid(360)) as f32;
        lcd.set_all_colors(color::hsv(hue, 1.0, 1.0));
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct Sparkle {
    frame: u32,
    active: HashMap<usize, (Rgb, i32)>,
}
impl Animation for Sparkle {
    fn update(&mut self, lcd: &mut LcdController) {
        let mut to_remove = Vec::new();
        for (&idx, entry) in self.active.iter_mut() {
            entry.1 -= 1;
            if entry.1 <= 0 {
                to_remove.push(idx);
            }
        }
        for idx in &to_remove {
            self.active.remove(idx);
            lcd.set_led(*idx, false);
        }

        let mut rng = rand::rng();
        let num_new = if rng.random::<f32>() < 0.22 {
            rng.random_range(1..=2)
        } else {
            0
        };
        for _ in 0..num_new {
            let idx = rng.random_range(0..NUMBER_OF_LEDS);
            let c = color::hsv(
                rng.random_range(0.0..360.0),
                rng.random_range(0.8..1.0),
                1.0,
            );
            let life = rng.random_range(18..45);
            self.active.insert(idx, (c, life));
            lcd.set_led(idx, true);
            lcd.set_color(idx, c);
        }
        self.frame = self.frame.wrapping_add(1);
    }

    fn reset(&mut self, lcd: &mut LcdController) {
        self.active.clear();
        lcd.clear();
    }
}

#[derive(Default)]
pub struct RandomBurst {
    frame: u32,
    next_burst: u32,
}
impl Animation for RandomBurst {
    fn update(&mut self, lcd: &mut LcdController) {
        let mut rng = rand::rng();
        lcd.set_all_leds(true);
        let base_hue = (self.frame as f32 * 0.35).rem_euclid(360.0);
        for i in 0..NUMBER_OF_LEDS {
            let wave = ((i as f32 * 0.11 + self.frame as f32 * 0.03).sin() * 0.5 + 0.5) * 0.10;
            lcd.set_color(i, color::hsv(base_hue + i as f32 * 0.8, 0.55, 0.03 + wave));
        }
        if self.next_burst == 0 || self.frame >= self.next_burst {
            let center = rng.random_range(0..NUMBER_OF_LEDS) as i32;
            let hue = rng.random_range(0.0..360.0);
            let radius = rng.random_range(8..=18);
            for off in -radius..=radius {
                let idx = center + off;
                if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                    let b = 1.0 - off.abs() as f32 / radius as f32;
                    lcd.set_color(
                        idx as usize,
                        color::hsv(hue + off as f32 * 2.0, 0.85, b.powf(1.4)),
                    );
                }
            }
            self.next_burst = self.frame + rng.random_range(22..=55);
        }
        self.frame = self.frame.wrapping_add(1);
    }

    fn preferred_duration(&self) -> f32 {
        11.0
    }
}

pub struct GradientSweep {
    frame: u32,
    gradient: Vec<Rgb>,
}
impl GradientSweep {
    pub fn new() -> Self {
        let colors: &[Rgb] = &[
            [0xff, 0, 0],
            [0xff, 0xff, 0],
            [0, 0xff, 0],
            [0, 0xff, 0xff],
            [0, 0, 0xff],
            [0xff, 0, 0xff],
        ];
        Self {
            frame: 0,
            gradient: color::gradient(colors, NUMBER_OF_LEDS),
        }
    }
}
impl Animation for GradientSweep {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        for i in 0..NUMBER_OF_LEDS {
            let idx = (i + self.frame as usize) % NUMBER_OF_LEDS;
            lcd.set_color(i, self.gradient[idx]);
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

pub struct Plasma {
    frame: u32,
    offset: f32,
    speed_mult: f32,
}
impl Plasma {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        Self {
            frame: 0,
            offset: rng.random_range(0.0..100.0),
            speed_mult: rng.random_range(0.8..1.2),
        }
    }
}
impl Animation for Plasma {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        if self.frame > 0 && self.frame % 200 == 0 {
            let mut rng = rand::rng();
            self.offset = rng.random_range(0.0..100.0);
            self.speed_mult = rng.random_range(0.8..1.2);
        }
        let f = self.frame as f32;
        let sm = self.speed_mult;
        for i in 0..NUMBER_OF_LEDS {
            let x = i as f32;
            let mut v = (x * 0.3 + f * 0.1 * sm + self.offset).sin();
            v += ((x * 0.2 + f * 0.15 * sm) * 1.5).sin();
            v += ((x * 0.1).powi(2) + (f * 0.08 * sm).powi(2)).sqrt().sin();
            let v = (v + 3.0) / 6.0;
            let hue = ((v * 360.0) as i32 + (self.offset * 3.0) as i32).rem_euclid(360) as f32;
            lcd.set_color(i, color::hsv(hue, 1.0, 1.0));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

struct MatrixDrop {
    pos: f32,
    speed: f32,
    length: i32,
    hue: i32,
}

pub struct MatrixRain {
    frame: u32,
    drops: Vec<MatrixDrop>,
}
impl MatrixRain {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let drops = (0..12)
            .map(|_| MatrixDrop {
                pos: rng.random_range(0..NUMBER_OF_LEDS as i32) as f32,
                speed: rng.random_range(0.5..2.5),
                length: rng.random_range(5..=15),
                hue: rng.random_range(100..=140),
            })
            .collect();
        Self { frame: 0, drops }
    }
}
impl Animation for MatrixRain {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut rng = rand::rng();
        for drop in &mut self.drops {
            drop.pos += drop.speed;
            if drop.pos > (NUMBER_OF_LEDS as f32 + drop.length as f32) {
                drop.pos = -(drop.length as f32);
                drop.speed = rng.random_range(0.5..2.5);
                drop.length = rng.random_range(5..=15);
                drop.hue = rng.random_range(100..=140);
            }
            for i in 0..drop.length {
                let idx = drop.pos as i32 - i;
                if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                    let b = 1.0 - (i as f32 / drop.length as f32);
                    let hue = (drop.hue + rng.random_range(-10..=10)).rem_euclid(360) as f32;
                    lcd.set_led(idx as usize, true);
                    lcd.set_color(idx as usize, color::hsv(hue, 1.0, b));
                }
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Display animations
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct SegmentCrawl {
    frame: u32,
}
impl Animation for SegmentCrawl {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let idx = (self.frame / 3) as usize % NUMBER_OF_LEDS;
        let tail: i32 = 15;
        for i in 0..tail {
            let led = (idx as i32 - i).rem_euclid(NUMBER_OF_LEDS as i32) as usize;
            let b = 1.0 - (i as f32 / tail as f32);
            let hue = ((self.frame as i64 + i as i64 * 5).rem_euclid(360)) as f32;
            lcd.set_led(led, true);
            lcd.set_color(led, color::hsv(hue, 1.0, b));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct ColorWipe {
    frame: u32,
}
impl Animation for ColorWipe {
    fn update(&mut self, lcd: &mut LcdController) {
        let cycle: u32 = NUMBER_OF_LEDS as u32 + 20;
        let pos = self.frame % cycle;
        if (pos as usize) < NUMBER_OF_LEDS {
            let hue = ((self.frame / cycle * 60) as i32).rem_euclid(360) as f32;
            let c = color::hsv(hue, 1.0, 1.0);
            lcd.set_led(pos as usize, true);
            lcd.set_color(pos as usize, c);
        } else {
            let clear_pos = (pos as usize).saturating_sub(NUMBER_OF_LEDS);
            if clear_pos < NUMBER_OF_LEDS {
                lcd.set_led(clear_pos, false);
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct RainbowSegments {
    frame: u32,
}
impl Animation for RainbowSegments {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let regions: [&[usize]; 12] = [
            CPU_LED,
            CPU_TEMP,
            &[CPU_CELSIUS[0], CPU_FAHRENHEIT[0]],
            CPU_USAGE_1_INDICATORS,
            CPU_USAGE,
            CPU_PERCENT_LED,
            GPU_PERCENT_LED,
            GPU_USAGE,
            GPU_USAGE_1_INDICATORS,
            &[GPU_CELSIUS[0], GPU_FAHRENHEIT[0]],
            GPU_TEMP,
            GPU_LED,
        ];
        for (region_idx, region) in regions.iter().enumerate() {
            let hue = ((self.frame as i64 * 2 + region_idx as i64 * 30).rem_euclid(360)) as f32;
            lcd.set_colors(region, color::hsv(hue, 1.0, 1.0));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}
