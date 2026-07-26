use std::collections::HashMap;
use std::f32::consts::PI;

use rand::prelude::IndexedRandom;
use rand::RngExt;

use super::Animation;
use crate::color::{self, Rgb};
use crate::lcd::LcdController;
use crate::led_map::{CPU_ALL, GPU_ALL, NUMBER_OF_LEDS};

// ---------------------------------------------------------------------------
// Pulse & strobe
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct Heartbeat {
    frame: u32,
}
impl Animation for Heartbeat {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let t = (self.frame % 100) as f32 / 100.0;
        let brightness = if t < 0.1 {
            (t * 10.0 * PI).sin()
        } else if (0.2..0.3).contains(&t) {
            ((t - 0.2) * 10.0 * PI).sin()
        } else {
            0.1
        };
        lcd.set_all_colors(color::hsv(0.0, 1.0, brightness.max(0.0)));
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct Lighthouse {
    frame: u32,
}
impl Animation for Lighthouse {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let beam_width: i32 = 10;
        let center = (self.frame * 2) as i32 % NUMBER_OF_LEDS as i32;
        for i in 0..beam_width {
            let idx = (center + i - beam_width / 2).rem_euclid(NUMBER_OF_LEDS as i32) as usize;
            let dist = (i - beam_width / 2).abs() as f32;
            let b = 1.0 - dist / (beam_width as f32 / 2.0);
            lcd.set_led(idx, true);
            lcd.set_color(idx, color::hsv(50.0, 0.8, b.max(0.0)));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Geometric
// ---------------------------------------------------------------------------

pub struct Snake {
    frame: u32,
    length: usize,
}
impl Default for Snake {
    fn default() -> Self {
        Self {
            frame: 0,
            length: 20,
        }
    }
}
impl Animation for Snake {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let head = (self.frame as usize) % NUMBER_OF_LEDS;
        for i in 0..self.length {
            let idx = (head as i32 - i as i32).rem_euclid(NUMBER_OF_LEDS as i32) as usize;
            let b = 1.0 - (i as f32 / self.length as f32);
            let hue = ((self.frame as i64 + i as i64 * 5).rem_euclid(360)) as f32;
            lcd.set_led(idx, true);
            lcd.set_color(idx, color::hsv(hue, 1.0, b));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

pub struct BouncingBall {
    frame: u32,
    position: f32,
    velocity: f32,
    gravity: f32,
}
impl BouncingBall {
    pub fn new() -> Self {
        Self {
            frame: 0,
            position: 0.0,
            velocity: 2.0,
            gravity: 0.2,
        }
    }
}
impl Animation for BouncingBall {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        self.velocity += self.gravity;
        self.position += self.velocity;
        if self.position >= (NUMBER_OF_LEDS - 1) as f32 {
            self.position = (NUMBER_OF_LEDS - 1) as f32;
            self.velocity = -self.velocity * 0.85;
        }
        if self.position < 0.0 {
            self.position = 0.0;
            self.velocity = -self.velocity;
        }
        if self.velocity.abs() < 0.5 {
            let mut rng = rand::rng();
            self.position = rng.random_range(0..(NUMBER_OF_LEDS - 20)) as f32;
            self.velocity = rng.random_range(1.5..3.0);
        }
        let size: i32 = 5;
        for i in 0..size {
            let idx = self.position as i32 - i;
            if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                let b = 1.0 - (i as f32 / size as f32);
                let hue = ((self.frame as i64 + i as i64 * 10).rem_euclid(360)) as f32;
                lcd.set_led(idx as usize, true);
                lcd.set_color(idx as usize, color::hsv(hue, 1.0, b));
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

pub struct PingPong {
    frame: u32,
    pos: f32,
    side_cpu: bool,
    vel: f32,
}
impl PingPong {
    pub fn new() -> Self {
        Self {
            frame: 0,
            pos: 0.0,
            side_cpu: true,
            vel: 2.0,
        }
    }
}
impl Animation for PingPong {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        self.pos += self.vel;
        if self.pos >= 41.0 {
            self.side_cpu = !self.side_cpu;
            self.pos = 0.0;
        }
        let region = if self.side_cpu { CPU_ALL } else { GPU_ALL };
        let p = self.pos as i32;
        let hue = if self.side_cpu { 48.0 } else { 188.0 };
        for tail in 0..8 {
            let idx = p - tail;
            if (0..region.len() as i32).contains(&idx) {
                let led = region[idx as usize];
                let b = (1.0 - tail as f32 / 8.0).powf(1.7);
                lcd.set_led(led, true);
                lcd.set_color(led, color::hsv(hue, 0.9, b));
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct Spiral {
    frame: u32,
}
impl Animation for Spiral {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        for i in 0..NUMBER_OF_LEDS {
            let angle = (i as f32 / NUMBER_OF_LEDS as f32) * 2.0 * PI;
            let offset = (angle * 3.0 + self.frame as f32 * 0.1).sin() * 0.5 + 0.5;
            let hue = offset * 360.0;
            lcd.set_color(i, color::hsv(hue, 1.0, 1.0));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Audio / music
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct VuMeter {
    frame: u32,
}
impl Animation for VuMeter {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let level_cpu = (self.frame as f32 * 0.15).sin().abs() * 40.0;
        let level_gpu = (self.frame as f32 * 0.12 + 1.0).sin().abs() * 40.0;
        let draw = |lcd: &mut LcdController, region: &[usize], level: f32| {
            for i in 0..level as usize {
                if i < region.len() {
                    let led = region[i];
                    let c = if i < 25 {
                        color::GREEN
                    } else if i < 35 {
                        color::YELLOW
                    } else {
                        color::RED
                    };
                    lcd.set_led(led, true);
                    lcd.set_color(led, c);
                }
            }
        };
        draw(lcd, CPU_ALL, level_cpu);
        draw(lcd, GPU_ALL, level_gpu);
        self.frame = self.frame.wrapping_add(1);
    }
}

pub struct Equalizer {
    frame: u32,
    band_heights: Vec<f32>,
}
impl Equalizer {
    pub fn new() -> Self {
        Self {
            frame: 0,
            band_heights: vec![0.0; 7],
        }
    }
}
impl Animation for Equalizer {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let nb = self.band_heights.len();
        for i in 0..nb {
            let target = (self.frame as f32 * 0.1 + i as f32 * 0.5).sin().abs() * 10.0;
            self.band_heights[i] = self.band_heights[i] * 0.7 + target * 0.3;
        }
        let per_band = NUMBER_OF_LEDS / nb;
        for band in 0..nb {
            let h = self.band_heights[band] as usize;
            let start = band * per_band;
            for i in 0..h {
                let idx = start + i;
                if idx < NUMBER_OF_LEDS {
                    let hue = ((band as i64 * 50 + self.frame as i64).rem_euclid(360)) as f32;
                    lcd.set_led(idx, true);
                    lcd.set_color(idx, color::hsv(hue, 1.0, 1.0));
                }
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct BeatPulse {
    frame: u32,
}
impl Animation for BeatPulse {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let beat = self.frame % 30;
        let brightness = if beat < 5 {
            1.0 - beat as f32 / 5.0
        } else {
            0.3
        };
        let hue = ((self.frame as i64 * 2).rem_euclid(360)) as f32;
        lcd.set_all_colors(color::hsv(hue, 1.0, brightness));
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Nature
// ---------------------------------------------------------------------------

pub struct Lightning {
    frame: u32,
    strike_frame: u32,
    next_strike: u32,
}
impl Lightning {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        Self {
            frame: 0,
            strike_frame: 0,
            next_strike: rng.random_range(50..=150),
        }
    }
}
impl Animation for Lightning {
    fn update(&mut self, lcd: &mut LcdController) {
        let mut rng = rand::rng();
        if self.frame >= self.next_strike {
            self.strike_frame = self.frame;
            self.next_strike = self.frame + rng.random_range(50..=150);
        }
        let age = self.frame - self.strike_frame;
        if age < 2 {
            lcd.set_all_leds(true);
            lcd.set_all_colors(color::WHITE);
        } else if age < 5 {
            lcd.set_all_leds(true);
            let b = 0.5 - (age - 2) as f32 * 0.15;
            lcd.set_all_colors(color::hsv(240.0, 0.3, b.max(0.0)));
        } else if rng.random::<f32>() < 0.05 {
            lcd.set_all_leds(true);
            lcd.set_all_colors([0x1a, 0x1a, 0x2e]);
        } else {
            lcd.clear();
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct Aurora {
    frame: u32,
}
impl Animation for Aurora {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        for i in 0..NUMBER_OF_LEDS {
            let v1 = (i as f32 * 0.2 + self.frame as f32 * 0.05).sin();
            let v2 = (i as f32 * 0.1 + self.frame as f32 * 0.03).sin();
            let b = ((v1 + v2) * 0.25 + 0.5).clamp(0.2, 1.0);
            let hue = (120.0 + v1 * 60.0).rem_euclid(360.0);
            lcd.set_color(i, color::hsv(hue, 0.8, b));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct Fireflies {
    frame: u32,
    fireflies: HashMap<usize, f32>,
}
impl Animation for Fireflies {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut to_remove = Vec::new();
        for (&idx, b) in self.fireflies.iter_mut() {
            *b -= 0.05;
            if *b <= 0.0 {
                to_remove.push(idx);
            } else {
                lcd.set_led(idx, true);
                lcd.set_color(idx, color::hsv(60.0, 1.0, *b));
            }
        }
        for idx in to_remove {
            self.fireflies.remove(&idx);
        }
        let mut rng = rand::rng();
        if rng.random::<f32>() < 0.15 {
            let idx = rng.random_range(0..NUMBER_OF_LEDS);
            self.fireflies.insert(idx, 1.0);
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct LavaLamp {
    frame: u32,
}
impl Animation for LavaLamp {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        for i in 0..NUMBER_OF_LEDS {
            let blob1 = (i as f32 * 0.3 + self.frame as f32 * 0.08).sin() * 0.5;
            let blob2 = (i as f32 * 0.2 + self.frame as f32 * 0.05 + 2.0).sin() * 0.5;
            let combined = blob1 + blob2;
            let b = (combined + 1.0) / 2.0;
            let hue = b * 60.0;
            lcd.set_color(i, color::hsv(hue, 1.0, 0.5 + b * 0.5));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Retro
// ---------------------------------------------------------------------------

pub struct Pacman {
    frame: u32,
    pacman_pos: i32,
    ghost_pos: i32,
}
impl Pacman {
    pub fn new() -> Self {
        Self {
            frame: 0,
            pacman_pos: 0,
            ghost_pos: -10,
        }
    }
}
impl Animation for Pacman {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        self.pacman_pos = (self.pacman_pos + 1) % (NUMBER_OF_LEDS as i32 + 20);
        self.ghost_pos = (self.ghost_pos + 1) % (NUMBER_OF_LEDS as i32 + 20);
        if (0..NUMBER_OF_LEDS as i32).contains(&self.pacman_pos) {
            lcd.set_led(self.pacman_pos as usize, true);
            lcd.set_color(self.pacman_pos as usize, color::YELLOW);
            if (self.frame % 10) < 5 {
                let behind = (self.pacman_pos - 1).rem_euclid(NUMBER_OF_LEDS as i32) as usize;
                lcd.set_led(behind, false);
            }
        }
        if (0..NUMBER_OF_LEDS as i32).contains(&self.ghost_pos) {
            let c = color::wheel(self.frame * 5);
            lcd.set_led(self.ghost_pos as usize, true);
            lcd.set_color(self.ghost_pos as usize, c);
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

struct TetrisBlock {
    pos: f32,
    color: Rgb,
    length: i32,
}

#[derive(Default)]
pub struct TetrisBlocks {
    frame: u32,
    blocks: Vec<TetrisBlock>,
}
impl Animation for TetrisBlocks {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut rng = rand::rng();
        if self.frame % 30 == 0 {
            self.blocks.push(TetrisBlock {
                pos: 0.0,
                color: color::wheel(rng.random_range(0..=255)),
                length: rng.random_range(3..=6),
            });
        }
        self.blocks.retain_mut(|b| {
            b.pos += 0.5;
            if b.pos > NUMBER_OF_LEDS as f32 {
                return false;
            }
            for i in 0..b.length {
                let idx = b.pos as i32 + i;
                if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                    lcd.set_led(idx as usize, true);
                    lcd.set_color(idx as usize, b.color);
                }
            }
            true
        });
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Advanced effects
// ---------------------------------------------------------------------------

pub struct Comet {
    frame: u32,
    tail: i32,
}
impl Default for Comet {
    fn default() -> Self {
        Self { frame: 0, tail: 25 }
    }
}
impl Animation for Comet {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let head = (self.frame * 2) as i32 % NUMBER_OF_LEDS as i32;
        for i in 0..self.tail {
            let idx = (head - i).rem_euclid(NUMBER_OF_LEDS as i32) as usize;
            let mut b = 1.0 - (i as f32 / self.tail as f32);
            b *= b;
            lcd.set_led(idx, true);
            let c = if i < 5 {
                color::WHITE
            } else {
                color::hsv(200.0, 1.0, b)
            };
            lcd.set_color(idx, c);
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

struct Explosion {
    center: i32,
    color: Rgb,
    age: i32,
}

#[derive(Default)]
pub struct Fireworks {
    frame: u32,
    explosions: Vec<Explosion>,
}
impl Animation for Fireworks {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut rng = rand::rng();
        if self.frame % 40 == 0 {
            self.explosions.push(Explosion {
                center: rng.random_range(10..(NUMBER_OF_LEDS as i32 - 10)),
                color: color::wheel(rng.random_range(0..=255)),
                age: 0,
            });
        }
        self.explosions.retain_mut(|e| {
            e.age += 1;
            if e.age > 30 {
                return false;
            }
            let radius = e.age;
            for i in -radius..=radius {
                let idx = e.center + i;
                if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                    lcd.set_led(idx as usize, true);
                    lcd.set_color(idx as usize, e.color);
                }
            }
            true
        });
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct Waterfall {
    frame: u32,
}
impl Animation for Waterfall {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        for i in 0..NUMBER_OF_LEDS {
            let wave = ((i as f32 - self.frame as f32) * 0.3).sin();
            let b = (wave + 1.0) / 2.0;
            let hue = 180.0 + b * 30.0;
            lcd.set_color(i, color::hsv(hue, 0.8, 0.5 + b * 0.5));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct DnaHelix {
    frame: u32,
}
impl Animation for DnaHelix {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        for i in 0..NUMBER_OF_LEDS {
            let s1 = ((i as f32 + self.frame as f32) * 0.3).sin();
            let s2 = ((i as f32 + self.frame as f32) * 0.3 + PI).sin();
            let c = if s1 > s2 {
                color::hsv(300.0, 1.0, 1.0)
            } else {
                color::hsv(180.0, 1.0, 1.0)
            };
            lcd.set_color(i, c);
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct RainbowSpiral {
    frame: u32,
}
impl Animation for RainbowSpiral {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        for i in 0..NUMBER_OF_LEDS {
            let angle = (i as f32 / NUMBER_OF_LEDS as f32) * PI * 4.0;
            let rotation = self.frame as f32 * 0.05;
            let hue = ((angle + rotation) * 180.0 / PI).rem_euclid(360.0);
            lcd.set_color(i, color::hsv(hue, 1.0, 1.0));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct MirrorBounce {
    frame: u32,
}
impl Animation for MirrorBounce {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let pos = ((self.frame / 2) % 42) as i32;
        for tail in 0..10 {
            let b = (1.0 - tail as f32 / 10.0).powf(1.5);
            let cpu = pos - tail;
            if (0..CPU_ALL.len() as i32).contains(&cpu) {
                lcd.set_led(CPU_ALL[cpu as usize], true);
                lcd.set_color(CPU_ALL[cpu as usize], color::hsv(308.0, 0.85, b));
            }
            let gpu = 41 - pos + tail;
            if (0..GPU_ALL.len() as i32).contains(&gpu) {
                lcd.set_led(GPU_ALL[gpu as usize], true);
                lcd.set_color(GPU_ALL[gpu as usize], color::hsv(184.0, 0.85, b));
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct Sunset {
    frame: u32,
}
impl Animation for Sunset {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let t = (self.frame % 200) as f32 / 200.0;
        let c = if t < 0.3 {
            color::interpolate(color::hex("87CEEB"), color::hex("FF6B6B"), t / 0.3)
        } else if t < 0.6 {
            color::interpolate(color::hex("FF6B6B"), color::hex("FF8C00"), (t - 0.3) / 0.3)
        } else if t < 0.8 {
            color::interpolate(color::hex("FF8C00"), color::hex("4A148C"), (t - 0.6) / 0.2)
        } else {
            color::interpolate(color::hex("4A148C"), color::hex("0A0A0A"), (t - 0.8) / 0.2)
        };
        lcd.set_all_colors(c);
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct ScanLine {
    frame: u32,
}
impl Animation for ScanLine {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        lcd.set_all_colors([0x22, 0x22, 0x22]);
        let pos = (self.frame as usize) % NUMBER_OF_LEDS;
        for i in -3..=3 {
            let idx = (pos as i32 + i).rem_euclid(NUMBER_OF_LEDS as i32) as usize;
            let b = 1.0 - (i.abs() as f32 / 3.0);
            lcd.set_led(idx, true);
            lcd.set_color(idx, color::hsv(180.0, 1.0, b));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct Kaleidoscope {
    frame: u32,
}
impl Animation for Kaleidoscope {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let mid = NUMBER_OF_LEDS / 2;
        for i in 0..mid {
            let hue = ((i as i64 * 10 + self.frame as i64 * 2).rem_euclid(360)) as f32;
            let c = color::hsv(hue, 1.0, 1.0);
            lcd.set_color(i, c);
            lcd.set_color(NUMBER_OF_LEDS - 1 - i, c);
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

struct MeteorItem {
    pos: f32,
    speed: f32,
}

#[derive(Default)]
pub struct Meteor {
    frame: u32,
    meteors: Vec<MeteorItem>,
}
impl Animation for Meteor {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut rng = rand::rng();
        if rng.random::<f32>() < 0.1 {
            self.meteors.push(MeteorItem {
                pos: 0.0,
                speed: rng.random_range(1.5..3.0),
            });
        }
        self.meteors.retain_mut(|m| {
            m.pos += m.speed;
            if m.pos > NUMBER_OF_LEDS as f32 + 10.0 {
                return false;
            }
            for i in 0..10 {
                let idx = m.pos as i32 - i;
                if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                    let b = 1.0 - (i as f32 / 10.0);
                    lcd.set_led(idx as usize, true);
                    lcd.set_color(idx as usize, color::hsv(30.0, 0.8, b));
                }
            }
            true
        });
        self.frame = self.frame.wrapping_add(1);
    }
}

pub struct RgbWindmills {
    frame: u32,
    num_mills: usize,
    blade_length: i32,
    rotation_speed: u32,
}
impl Default for RgbWindmills {
    fn default() -> Self {
        Self {
            frame: 0,
            num_mills: 12,
            blade_length: 3,
            rotation_speed: 8,
        }
    }
}
impl Animation for RgbWindmills {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        for mill_idx in 0..self.num_mills {
            let center = (((mill_idx as f32 + 0.5)
                * (NUMBER_OF_LEDS as f32 / self.num_mills as f32))
                as i32)
                .min(NUMBER_OF_LEDS as i32 - 1);
            let rotation = (self.frame / self.rotation_speed + mill_idx as u32) % 4;
            let hue = ((mill_idx as i64 * 30 + self.frame as i64).rem_euclid(360)) as f32;
            let c = color::hsv(hue, 1.0, 1.0);

            for blade in 0..4u32 {
                let bp = (rotation + blade) % 4;
                match bp {
                    0 => {
                        for i in 1..=self.blade_length {
                            let idx = center + i;
                            if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                                lcd.set_led(idx as usize, true);
                                lcd.set_color(idx as usize, c);
                            }
                        }
                    }
                    1 => {
                        let idx = center + 1;
                        if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                            lcd.set_led(idx as usize, true);
                            lcd.set_color(idx as usize, c);
                        }
                    }
                    2 => {
                        for i in 1..=self.blade_length {
                            let idx = center - i;
                            if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                                lcd.set_led(idx as usize, true);
                                lcd.set_color(idx as usize, c);
                            }
                        }
                    }
                    _ => {
                        let idx = center - 1;
                        if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                            lcd.set_led(idx as usize, true);
                            lcd.set_color(idx as usize, c);
                        }
                    }
                }
            }
            if (0..NUMBER_OF_LEDS as i32).contains(&center) {
                lcd.set_led(center as usize, true);
                lcd.set_color(center as usize, c);
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

struct Bubble {
    pos: f32,
    speed: f32,
    color: Rgb,
    size: i32,
}

#[derive(Default)]
pub struct Bubbles {
    frame: u32,
    bubbles: Vec<Bubble>,
}
impl Animation for Bubbles {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut rng = rand::rng();
        if rng.random::<f32>() < 0.2 {
            self.bubbles.push(Bubble {
                pos: (NUMBER_OF_LEDS - 1) as f32,
                speed: rng.random_range(0.3..1.2),
                color: color::hsv(rng.random_range(180.0..240.0), 0.7, 1.0),
                size: rng.random_range(2..=4),
            });
        }
        self.bubbles.retain_mut(|b| {
            b.pos -= b.speed;
            if b.pos < -(b.size as f32) {
                return false;
            }
            for i in 0..b.size {
                let idx = b.pos as i32 + i;
                if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                    lcd.set_led(idx as usize, true);
                    lcd.set_color(idx as usize, b.color);
                }
            }
            true
        });
        self.frame = self.frame.wrapping_add(1);
    }
}

struct Star {
    brightness: f32,
    speed: f32,
    direction: i32,
}

pub struct Stars {
    frame: u32,
    stars: HashMap<usize, Star>,
}
impl Stars {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut stars = HashMap::new();
        for _ in 0..15 {
            let idx = rng.random_range(0..NUMBER_OF_LEDS);
            stars.insert(
                idx,
                Star {
                    brightness: rng.random::<f32>(),
                    speed: rng.random_range(0.02..0.08),
                    direction: *[1, -1].choose(&mut rng).unwrap(),
                },
            );
        }
        Self { frame: 0, stars }
    }
}
impl Animation for Stars {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut rng = rand::rng();
        if rng.random::<f32>() < 0.05 && self.stars.len() < 20 {
            let idx = rng.random_range(0..NUMBER_OF_LEDS);
            self.stars.insert(
                idx,
                Star {
                    brightness: rng.random::<f32>(),
                    speed: rng.random_range(0.02..0.08),
                    direction: *[1, -1].choose(&mut rng).unwrap(),
                },
            );
        }
        for (&idx, s) in self.stars.iter_mut() {
            s.brightness += s.speed * s.direction as f32;
            if s.brightness >= 1.0 {
                s.brightness = 1.0;
                s.direction = -1;
            } else if s.brightness <= 0.1 {
                s.brightness = 0.1;
                s.direction = 1;
            }
            lcd.set_led(idx, true);
            lcd.set_color(idx, color::hsv(45.0, 0.3, s.brightness));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

struct WarpStar {
    pos: f32,
    speed: f32,
    direction: i32,
}

#[derive(Default)]
pub struct WarpSpeed {
    frame: u32,
    stars: Vec<WarpStar>,
}
impl Animation for WarpSpeed {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut rng = rand::rng();
        if self.frame % 2 == 0 {
            self.stars.push(WarpStar {
                pos: (NUMBER_OF_LEDS / 2) as f32,
                speed: 1.0,
                direction: *[1, -1].choose(&mut rng).unwrap(),
            });
        }
        self.stars.retain_mut(|s| {
            s.pos += s.speed * s.direction as f32;
            s.speed *= 1.1;
            if s.pos < 0.0 || s.pos >= NUMBER_OF_LEDS as f32 {
                return false;
            }
            let streak = (s.speed as i32).clamp(1, 8);
            for i in 0..streak {
                let idx = s.pos as i32 - i * s.direction;
                if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                    let b = 1.0 - (i as f32 / streak as f32);
                    lcd.set_led(idx as usize, true);
                    lcd.set_color(idx as usize, color::hsv(200.0, 0.5, b));
                }
            }
            true
        });
        self.frame = self.frame.wrapping_add(1);
    }
}

struct BinColumn {
    pos: f32,
    speed: f32,
    bits: [u8; 10],
}

pub struct BinaryRain {
    frame: u32,
    columns: Vec<BinColumn>,
}
impl BinaryRain {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let mut columns = Vec::new();
        let mut i = 0;
        while i < NUMBER_OF_LEDS {
            let mut bits = [0u8; 10];
            for b in &mut bits {
                *b = rng.random_range(0..=1);
            }
            columns.push(BinColumn {
                pos: rng.random_range(-20.0..0.0),
                speed: rng.random_range(0.5..1.5),
                bits,
            });
            i += 4;
        }
        Self { frame: 0, columns }
    }
}
impl Animation for BinaryRain {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut rng = rand::rng();
        for col in &mut self.columns {
            col.pos += col.speed;
            if col.pos > NUMBER_OF_LEDS as f32 + 10.0 {
                col.pos = -10.0;
                for b in &mut col.bits {
                    *b = rng.random_range(0..=1);
                }
            }
            for (i, &bit) in col.bits.iter().enumerate() {
                let idx = col.pos as i32 + i as i32;
                if (0..NUMBER_OF_LEDS as i32).contains(&idx) && bit == 1 {
                    let b = if i == 0 { 1.0 } else { 0.5 };
                    lcd.set_led(idx as usize, true);
                    lcd.set_color(idx as usize, color::hsv(120.0, 1.0, b));
                }
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

struct Ring {
    center: i32,
    radius: f32,
    color: Rgb,
}

#[derive(Default)]
pub struct PulseRing {
    frame: u32,
    rings: Vec<Ring>,
}
impl Animation for PulseRing {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut rng = rand::rng();
        if self.frame % 20 == 0 {
            self.rings.push(Ring {
                center: NUMBER_OF_LEDS as i32 / 2,
                radius: 0.0,
                color: color::wheel(rng.random_range(0..=255)),
            });
        }
        self.rings.retain_mut(|r| {
            r.radius += 1.5;
            if r.radius > NUMBER_OF_LEDS as f32 / 2.0 + 5.0 {
                return false;
            }
            for off in [-(r.radius as i32), r.radius as i32] {
                let idx = r.center + off;
                if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                    lcd.set_led(idx as usize, true);
                    lcd.set_color(idx as usize, r.color);
                }
            }
            true
        });
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct ColorShift {
    frame: u32,
}
impl Animation for ColorShift {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let band_w: i64 = 9;
        let t = self.frame as f32 * 0.035;
        for i in 0..NUMBER_OF_LEDS {
            let band_idx = ((i as i64 + (self.frame / 4) as i64) / band_w) % 6;
            let soft_edge = ((i as f32 * 0.18 + t).sin() * 0.5 + 0.5) * 18.0;
            let hue = ((band_idx * 42) as f32 + soft_edge + t * 20.0).rem_euclid(360.0);
            lcd.set_color(i, color::hsv(hue, 0.82, 0.82));
        }
        self.frame = self.frame.wrapping_add(1);
    }

    fn preferred_duration(&self) -> f32 {
        12.0
    }
}

struct Walker {
    pos: i32,
    hue: i32,
}

pub struct RandomWalk {
    frame: u32,
    walkers: Vec<Walker>,
}
impl RandomWalk {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let walkers = (0..8)
            .map(|_| Walker {
                pos: rng.random_range(0..NUMBER_OF_LEDS as i32),
                hue: rng.random_range(0..360),
            })
            .collect();
        Self { frame: 0, walkers }
    }
}
impl Animation for RandomWalk {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut rng = rand::rng();
        for w in &mut self.walkers {
            w.pos = (w.pos + rng.random_range(-2..=2)).rem_euclid(NUMBER_OF_LEDS as i32);
            w.hue = (w.hue + rng.random_range(-5..=5)).rem_euclid(360);
            for i in 0..5 {
                let idx = (w.pos - i).rem_euclid(NUMBER_OF_LEDS as i32) as usize;
                let b = 1.0 - (i as f32 / 5.0);
                lcd.set_led(idx, true);
                lcd.set_color(idx, color::hsv(w.hue as f32, 1.0, b));
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct Glitch {
    frame: u32,
}
impl Animation for Glitch {
    fn update(&mut self, lcd: &mut LcdController) {
        let mut rng = rand::rng();
        if rng.random::<f32>() < 0.1 {
            lcd.clear();
            let n = rng.random_range(20..=40);
            for _ in 0..n {
                let idx = rng.random_range(0..NUMBER_OF_LEDS);
                let c = color::wheel(rng.random_range(0..=255));
                lcd.set_led(idx, true);
                lcd.set_color(idx, c);
            }
        } else {
            let n = rng.random_range(2..=8);
            for _ in 0..n {
                let idx = rng.random_range(0..NUMBER_OF_LEDS);
                if rng.random::<f32>() < 0.5 {
                    lcd.set_led(idx, false);
                } else {
                    let c = color::wheel(rng.random_range(0..=255));
                    lcd.set_led(idx, true);
                    lcd.set_color(idx, c);
                }
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

struct Scanner {
    pos: i32,
    direction: i32,
    hue: i32,
}

pub struct ScannerSweep {
    frame: u32,
    scanners: Vec<Scanner>,
}
impl ScannerSweep {
    pub fn new() -> Self {
        let mut rng = rand::rng();
        let scanners = (0..4)
            .map(|i| Scanner {
                pos: (i * (NUMBER_OF_LEDS / 4)) as i32,
                direction: *[1, -1].choose(&mut rng).unwrap(),
                hue: i as i32 * 90,
            })
            .collect();
        Self { frame: 0, scanners }
    }
}
impl Animation for ScannerSweep {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        for s in &mut self.scanners {
            s.pos += s.direction * 2;
            if s.pos <= 0 || s.pos >= NUMBER_OF_LEDS as i32 - 1 {
                s.direction = -s.direction;
                s.hue = (s.hue + 30).rem_euclid(360);
            }
            for i in -6..=6 {
                let idx = s.pos + i;
                if (0..NUMBER_OF_LEDS as i32).contains(&idx) {
                    let b = 1.0 - (i.abs() as f32 / 6.0);
                    lcd.set_led(idx as usize, true);
                    lcd.set_color(idx as usize, color::hsv(s.hue as f32, 1.0, b));
                }
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

struct ConfettiPiece {
    idx: usize,
    color: Rgb,
    life: i32,
}

#[derive(Default)]
pub struct Confetti {
    frame: u32,
    pieces: Vec<ConfettiPiece>,
}
impl Animation for Confetti {
    fn update(&mut self, lcd: &mut LcdController) {
        let mut rng = rand::rng();
        self.pieces.retain_mut(|p| {
            p.life -= 1;
            if p.life <= 0 {
                lcd.set_led(p.idx, false);
                false
            } else {
                let b = (p.life as f32 / 35.0).clamp(0.0, 1.0);
                lcd.set_color(p.idx, color::scale(p.color, b));
                true
            }
        });
        if rng.random::<f32>() < 0.3 {
            let n = rng.random_range(3..=8);
            for _ in 0..n {
                let idx = rng.random_range(0..NUMBER_OF_LEDS);
                let c = color::wheel(rng.random_range(0..=255));
                let life = rng.random_range(20..=40);
                self.pieces.push(ConfettiPiece {
                    idx,
                    color: c,
                    life,
                });
                lcd.set_led(idx, true);
                lcd.set_color(idx, c);
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

#[derive(Default)]
pub struct Ripple {
    frame: u32,
}
impl Animation for Ripple {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let n = NUMBER_OF_LEDS as f32;
        for i in 0..NUMBER_OF_LEDS {
            let f = self.frame as f32;
            let v1 = ((i as f32 - n / 3.0 - f * 0.5) * 0.3).sin();
            let v2 = ((i as f32 - n * 2.0 / 3.0 - f * 0.5) * 0.3).sin();
            let combined = (v1 + v2) / 2.0;
            let b = (combined + 1.0) / 2.0;
            let hue = 180.0 + b * 60.0;
            lcd.set_color(i, color::hsv(hue, 0.8, 0.5 + b * 0.5));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}
