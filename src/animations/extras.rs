use std::f32::consts::{PI, TAU};

use rand::Rng;

use super::Animation;
use crate::color::{self, Rgb};
use crate::lcd::LcdController;
use crate::led_map::NUMBER_OF_LEDS;

// ---------------------------------------------------------------------------
// Perlin-like value noise field
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct PerlinField {
    frame: u32,
    gradients: [f32; 16],
    initialized: bool,
}

impl PerlinField {
    fn smoothstep(t: f32) -> f32 {
        t * t * (3.0 - 2.0 * t)
    }

    fn sample(&self, x: f32) -> f32 {
        let n = self.gradients.len();
        let xi = x.floor() as i32;
        let xf = x - xi as f32;
        let g0 = self.gradients[(xi.rem_euclid(n as i32)) as usize];
        let g1 = self.gradients[((xi + 1).rem_euclid(n as i32)) as usize];
        let t = Self::smoothstep(xf);
        g0 + (g1 - g0) * t
    }
}

impl Animation for PerlinField {
    fn update(&mut self, lcd: &mut LcdController) {
        if !self.initialized {
            let mut rng = rand::thread_rng();
            for g in &mut self.gradients {
                *g = rng.gen_range(0.0..1.0);
            }
            self.initialized = true;
        }
        lcd.set_all_leds(true);
        let t = self.frame as f32 * 0.02;
        for i in 0..NUMBER_OF_LEDS {
            let x = i as f32 * 0.18;
            let v = self.sample(x + t) * 0.6 + self.sample(x * 2.3 - t * 1.3) * 0.4;
            let hue = (v * 540.0 + self.frame as f32 * 0.3).rem_euclid(360.0);
            let brightness = 0.5 + (v * 0.5).clamp(0.0, 0.5);
            lcd.set_color(i, color::hsv(hue, 0.9, brightness));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Two-vortex fluid swirl
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct FluidSwirl {
    frame: u32,
}

impl Animation for FluidSwirl {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let t = self.frame as f32;
        let cx1 = 20.0 + (t * 0.013).sin() * 10.0;
        let cx2 = 64.0 + (t * 0.009).cos() * 10.0;
        for i in 0..NUMBER_OF_LEDS {
            let x = i as f32;
            let d1 = x - cx1;
            let d2 = x - cx2;
            let s = (d1 * 0.4 + t * 0.07).sin() * (1.0 / (1.0 + d1.abs() * 0.1))
                + (d2 * 0.3 - t * 0.05).sin() * (1.0 / (1.0 + d2.abs() * 0.1));
            let hue = ((s * 180.0) + t * 0.4).rem_euclid(360.0);
            let b = (0.55 + s * 0.45).clamp(0.15, 1.0);
            lcd.set_color(i, color::hsv(hue, 1.0, b));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// 1D cellular automaton (rule 110 by default)
// ---------------------------------------------------------------------------

pub struct GameOfLife {
    frame: u32,
    state: [u8; NUMBER_OF_LEDS],
    next: [u8; NUMBER_OF_LEDS],
    rule: u8,
    step_every: u32,
    age: [u8; NUMBER_OF_LEDS],
}

impl GameOfLife {
    pub fn new() -> Self {
        let mut state = [0u8; NUMBER_OF_LEDS];
        let mut rng = rand::thread_rng();
        for s in &mut state {
            *s = rng.gen_range(0..=1);
        }
        Self {
            frame: 0,
            state,
            next: [0u8; NUMBER_OF_LEDS],
            rule: 110,
            step_every: 6,
            age: [0u8; NUMBER_OF_LEDS],
        }
    }
}

impl Animation for GameOfLife {
    fn update(&mut self, lcd: &mut LcdController) {
        if self.frame % self.step_every == 0 {
            for i in 0..NUMBER_OF_LEDS {
                let l = self.state[(i + NUMBER_OF_LEDS - 1) % NUMBER_OF_LEDS];
                let c = self.state[i];
                let r = self.state[(i + 1) % NUMBER_OF_LEDS];
                let pattern = (l << 2) | (c << 1) | r;
                self.next[i] = (self.rule >> pattern) & 1;
            }
            // Occasional perturbation to keep things alive
            if self.frame % 240 == 0 {
                let mut rng = rand::thread_rng();
                let i = rng.gen_range(0..NUMBER_OF_LEDS);
                self.next[i] ^= 1;
            }
            for i in 0..NUMBER_OF_LEDS {
                if self.next[i] == 1 && self.state[i] == 1 {
                    self.age[i] = self.age[i].saturating_add(1);
                } else if self.next[i] == 0 {
                    self.age[i] = 0;
                }
            }
            self.state = self.next;
        }
        lcd.clear();
        for i in 0..NUMBER_OF_LEDS {
            if self.state[i] == 1 {
                let hue = (i as f32 * 4.5 + self.frame as f32 * 0.4 + self.age[i] as f32 * 18.0)
                    .rem_euclid(360.0);
                lcd.set_led(i, true);
                lcd.set_color(i, color::hsv(hue, 1.0, 1.0));
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }

    fn reset(&mut self, lcd: &mut LcdController) {
        let mut rng = rand::thread_rng();
        for s in &mut self.state {
            *s = rng.gen_range(0..=1);
        }
        self.age = [0u8; NUMBER_OF_LEDS];
        lcd.clear();
    }
}

// ---------------------------------------------------------------------------
// Ferrofluid spikes
// ---------------------------------------------------------------------------

pub struct Ferrofluid {
    frame: u32,
    spikes: [(f32, f32, f32); 6], // (position, phase, hue_seed)
}

impl Default for Ferrofluid {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let mut spikes = [(0.0, 0.0, 0.0); 6];
        for (i, s) in spikes.iter_mut().enumerate() {
            s.0 = (i as f32 + 0.5) * (NUMBER_OF_LEDS as f32 / 6.0);
            s.1 = rng.gen_range(0.0..TAU);
            s.2 = rng.gen_range(0.0..360.0);
        }
        Self { frame: 0, spikes }
    }
}

impl Animation for Ferrofluid {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let t = self.frame as f32 * 0.08;
        for i in 0..NUMBER_OF_LEDS {
            let mut total = 0.0f32;
            let mut hue_acc = 0.0f32;
            let mut weight = 0.0f32;
            for &(pos, phase, hue_seed) in &self.spikes {
                let amp = ((t + phase).sin() * 0.5 + 0.6).max(0.1);
                let dist = (i as f32 - pos).abs();
                let falloff = (-(dist * dist) / (12.0 * amp)).exp();
                total += amp * falloff;
                hue_acc += hue_seed * falloff;
                weight += falloff;
            }
            let b = total.clamp(0.05, 1.0);
            let hue = if weight > 0.0 {
                ((hue_acc / weight) + t * 5.0).rem_euclid(360.0)
            } else {
                0.0
            };
            lcd.set_color(i, color::hsv(hue, 0.85, b));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Color volcano
// ---------------------------------------------------------------------------

struct Particle {
    pos: f32,
    vel: f32,
    color: Rgb,
    life: i32,
}

#[derive(Default)]
pub struct ColorVolcano {
    frame: u32,
    particles: Vec<Particle>,
}

impl Animation for ColorVolcano {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut rng = rand::thread_rng();
        // Erupt
        if self.frame % 6 == 0 {
            let n = rng.gen_range(1..=3);
            let center = NUMBER_OF_LEDS as f32 / 2.0;
            for _ in 0..n {
                self.particles.push(Particle {
                    pos: center + rng.gen_range(-2.0..2.0),
                    vel: rng.gen_range(-2.5..2.5),
                    color: color::hsv(rng.gen_range(0.0..60.0), 1.0, 1.0),
                    life: rng.gen_range(20..40),
                });
            }
        }
        // Glow base
        for i in 0..NUMBER_OF_LEDS {
            let d = (i as f32 - NUMBER_OF_LEDS as f32 / 2.0).abs();
            let b = (1.0 - d / 6.0).max(0.0) * 0.5;
            if b > 0.05 {
                lcd.set_led(i, true);
                lcd.set_color(i, color::hsv(20.0, 1.0, b));
            }
        }
        self.particles.retain_mut(|p| {
            p.life -= 1;
            p.pos += p.vel;
            p.vel *= 0.92;
            if p.life <= 0 || p.pos < 0.0 || p.pos >= NUMBER_OF_LEDS as f32 {
                return false;
            }
            let idx = p.pos as usize;
            lcd.set_led(idx, true);
            lcd.set_color(idx, p.color);
            true
        });
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Double pendulum (chaotic trace)
// ---------------------------------------------------------------------------

pub struct DoublePendulum {
    frame: u32,
    theta1: f32,
    theta2: f32,
    omega1: f32,
    omega2: f32,
    trail: Vec<(usize, f32, f32)>, // (idx, brightness, hue)
}

impl DoublePendulum {
    pub fn new() -> Self {
        Self {
            frame: 0,
            theta1: 2.4,
            theta2: 1.6,
            omega1: 0.0,
            omega2: 0.0,
            trail: Vec::with_capacity(64),
        }
    }
}

impl Animation for DoublePendulum {
    fn update(&mut self, lcd: &mut LcdController) {
        let g = 0.6_f32;
        let m1 = 1.0_f32;
        let m2 = 1.0_f32;
        let l1 = 1.0_f32;
        let l2 = 1.0_f32;
        let dt = 0.12_f32;
        let dtheta = self.theta1 - self.theta2;
        let denom1 = (2.0 * m1 + m2 - m2 * (2.0 * dtheta).cos()).max(1e-6);
        let num1 = -g * (2.0 * m1 + m2) * self.theta1.sin()
            - m2 * g * (self.theta1 - 2.0 * self.theta2).sin()
            - 2.0
                * dtheta.sin()
                * m2
                * (self.omega2.powi(2) * l2 + self.omega1.powi(2) * l1 * dtheta.cos());
        let a1 = num1 / (l1 * denom1);
        let num2 = 2.0
            * dtheta.sin()
            * (self.omega1.powi(2) * l1 * (m1 + m2)
                + g * (m1 + m2) * self.theta1.cos()
                + self.omega2.powi(2) * l2 * m2 * dtheta.cos());
        let a2 = num2 / (l2 * denom1);
        self.omega1 += a1 * dt;
        self.omega2 += a2 * dt;
        self.theta1 += self.omega1 * dt;
        self.theta2 += self.omega2 * dt;

        let x = l1 * self.theta1.sin() + l2 * self.theta2.sin();
        let center = NUMBER_OF_LEDS as f32 / 2.0;
        let scale = (NUMBER_OF_LEDS as f32 / 2.0) / (l1 + l2 + 0.5);
        let idx = (center + x * scale).clamp(0.0, (NUMBER_OF_LEDS - 1) as f32) as usize;
        let hue = ((self.theta2 * 60.0) + self.frame as f32 * 0.5).rem_euclid(360.0);
        self.trail.push((idx, 1.0, hue));
        if self.trail.len() > 50 {
            self.trail.remove(0);
        }

        lcd.clear();
        for (age, (i, _b, h)) in self.trail.iter().enumerate() {
            let b = age as f32 / self.trail.len() as f32;
            lcd.set_led(*i, true);
            lcd.set_color(*i, color::hsv(*h, 1.0, b.max(0.05)));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Wormhole (concentric expanding rings)
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct Wormhole {
    frame: u32,
}

impl Animation for Wormhole {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let t = self.frame as f32;
        let center = NUMBER_OF_LEDS as f32 / 2.0;
        for i in 0..NUMBER_OF_LEDS {
            let d = (i as f32 - center).abs();
            let phase = d * 0.45 - t * 0.18;
            let v = phase.sin() * 0.5 + 0.5;
            let hue = (d * 12.0 + t * 1.5).rem_euclid(360.0);
            let b = (v * 0.7 + 0.2) * (1.0 - d / (NUMBER_OF_LEDS as f32)).max(0.3);
            lcd.set_color(i, color::hsv(hue, 1.0, b));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Starfield warp (3D projected to 1D)
// ---------------------------------------------------------------------------

struct StarPoint {
    angle: f32, // radial angle 0..2pi
    z: f32,     // depth, decreases over time
    hue: f32,
}

pub struct StarfieldWarp {
    frame: u32,
    stars: Vec<StarPoint>,
}

impl StarfieldWarp {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let stars = (0..40)
            .map(|_| StarPoint {
                angle: rng.gen_range(0.0..TAU),
                z: rng.gen_range(0.1..1.0),
                hue: rng.gen_range(0.0..360.0),
            })
            .collect();
        Self { frame: 0, stars }
    }
}

impl Animation for StarfieldWarp {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.clear();
        let mut rng = rand::thread_rng();
        for s in &mut self.stars {
            s.z -= 0.012;
            if s.z <= 0.02 {
                s.angle = rng.gen_range(0.0..TAU);
                s.z = 1.0;
                s.hue = rng.gen_range(0.0..360.0);
            }
            let r = s.angle.cos();
            let projected = (NUMBER_OF_LEDS as f32 / 2.0) + r * (1.0 / s.z) * 8.0;
            if projected >= 0.0 && projected < NUMBER_OF_LEDS as f32 {
                let idx = projected as usize;
                let b = (1.0 - s.z).clamp(0.1, 1.0);
                lcd.set_led(idx, true);
                lcd.set_color(idx, color::hsv(s.hue, 0.7, b));
                // small streak toward the center for fast-moving stars
                let streak = ((1.0 - s.z) * 6.0) as i32;
                let center = NUMBER_OF_LEDS as i32 / 2;
                let dir = (idx as i32 - center).signum();
                for k in 1..streak {
                    let p = idx as i32 - dir * k;
                    if (0..NUMBER_OF_LEDS as i32).contains(&p) {
                        let bb = (b - k as f32 * 0.15).max(0.0);
                        if bb > 0.05 {
                            lcd.set_led(p as usize, true);
                            lcd.set_color(p as usize, color::hsv(s.hue, 0.5, bb));
                        }
                    }
                }
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Polyrhythmic drum circle
// ---------------------------------------------------------------------------

struct Drum {
    period: u32,
    phase: f32,
    hue: f32,
}

pub struct DrumCircle {
    frame: u32,
    drums: Vec<Drum>,
}

impl DrumCircle {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let drums = [7u32, 11, 13, 17, 23]
            .iter()
            .map(|&p| Drum {
                period: p,
                phase: rng.gen_range(0.0..1.0),
                hue: rng.gen_range(0.0..360.0),
            })
            .collect();
        Self { frame: 0, drums }
    }
}

impl Animation for DrumCircle {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        lcd.set_all_colors([4, 4, 8]);
        for d in &self.drums {
            let cycle = (self.frame as f32 + d.phase * d.period as f32) / d.period as f32;
            let pulse = ((cycle * TAU).sin() * 0.5 + 0.5).powi(3);
            let center = ((cycle * NUMBER_OF_LEDS as f32) as i32).rem_euclid(NUMBER_OF_LEDS as i32);
            let width = 6;
            for off in -width..=width {
                let idx = (center + off).rem_euclid(NUMBER_OF_LEDS as i32) as usize;
                let b = (1.0 - off.abs() as f32 / width as f32) * pulse;
                if b > 0.05 {
                    lcd.set_led(idx, true);
                    lcd.set_color(idx, color::hsv(d.hue, 1.0, b));
                }
            }
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Two-source wave interference
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct Interference {
    frame: u32,
}

impl Animation for Interference {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let t = self.frame as f32 * 0.12;
        let s1 = 18.0 + (t * 0.21).sin() * 8.0;
        let s2 = 66.0 + (t * 0.17).cos() * 8.0;
        for i in 0..NUMBER_OF_LEDS {
            let x = i as f32;
            let a = ((x - s1).abs() * 0.6 - t).sin();
            let b = ((x - s2).abs() * 0.6 - t * 1.05).sin();
            let interference = (a + b) / 2.0;
            let brightness = interference.abs().clamp(0.0, 1.0);
            let hue = (interference * 120.0 + 200.0 + t * 4.0).rem_euclid(360.0);
            lcd.set_color(i, color::hsv(hue, 0.9, 0.2 + brightness * 0.8));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Magnetic field between two poles
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct MagneticField {
    frame: u32,
}

impl Animation for MagneticField {
    fn update(&mut self, lcd: &mut LcdController) {
        lcd.set_all_leds(true);
        let t = self.frame as f32 * 0.04;
        let north = 22.0 + t.sin() * 6.0;
        let south = 62.0 + (t * 0.7).cos() * 6.0;
        for i in 0..NUMBER_OF_LEDS {
            let x = i as f32;
            let dn = (x - north).abs().max(1.0);
            let ds = (x - south).abs().max(1.0);
            let field = (1.0 / (dn * dn)) - (1.0 / (ds * ds));
            let intensity = field.abs() * 80.0;
            let brightness = intensity.clamp(0.05, 1.0);
            let hue = if field > 0.0 { 0.0 } else { 220.0 };
            lcd.set_color(i, color::hsv(hue, 0.9, brightness));
        }
        self.frame = self.frame.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// Predator-prey (Lotka-Volterra)
// ---------------------------------------------------------------------------

pub struct PredatorPrey {
    frame: u32,
    prey: f32,
    pred: f32,
    history: Vec<(f32, f32)>,
}

impl PredatorPrey {
    pub fn new() -> Self {
        Self {
            frame: 0,
            prey: 1.2,
            pred: 0.7,
            history: Vec::with_capacity(NUMBER_OF_LEDS),
        }
    }
}

impl Animation for PredatorPrey {
    fn update(&mut self, lcd: &mut LcdController) {
        // Lotka-Volterra
        let alpha = 0.10_f32;
        let beta = 0.04_f32;
        let delta = 0.02_f32;
        let gamma = 0.10_f32;
        let dt = 1.0_f32;
        let dprey = (alpha * self.prey - beta * self.prey * self.pred) * dt;
        let dpred = (delta * self.prey * self.pred - gamma * self.pred) * dt;
        self.prey += dprey;
        self.pred += dpred;
        self.prey = self.prey.max(0.01);
        self.pred = self.pred.max(0.01);
        self.history.push((self.prey, self.pred));
        if self.history.len() > NUMBER_OF_LEDS {
            self.history.remove(0);
        }
        lcd.set_all_leds(true);
        let mut max_prey: f32 = 0.001;
        let mut max_pred: f32 = 0.001;
        for &(p, q) in &self.history {
            if p > max_prey {
                max_prey = p;
            }
            if q > max_pred {
                max_pred = q;
            }
        }
        let m = max_prey.max(max_pred);
        for i in 0..NUMBER_OF_LEDS {
            let idx_h = i + NUMBER_OF_LEDS.saturating_sub(self.history.len());
            if idx_h < NUMBER_OF_LEDS && i >= NUMBER_OF_LEDS.saturating_sub(self.history.len()) {
                let h_idx = i + self.history.len() - NUMBER_OF_LEDS;
                let (p, q) = self.history[h_idx.min(self.history.len() - 1)];
                let red = (q / m).clamp(0.0, 1.0);
                let green = (p / m).clamp(0.0, 1.0);
                lcd.set_color(
                    i,
                    [(red * 255.0) as u8, (green * 255.0) as u8, ((red + green) * 40.0) as u8],
                );
            } else {
                lcd.set_color(i, [0, 0, 0]);
            }
        }
        let _ = PI;
        self.frame = self.frame.wrapping_add(1);
    }
}
