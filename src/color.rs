#![allow(dead_code)]

use rand::Rng;

pub type Rgb = [u8; 3];

pub const BLACK: Rgb = [0, 0, 0];
pub const WHITE: Rgb = [0xff, 0xff, 0xff];
pub const RED: Rgb = [0xff, 0, 0];
pub const GREEN: Rgb = [0, 0xff, 0];
pub const BLUE: Rgb = [0, 0, 0xff];
pub const YELLOW: Rgb = [0xff, 0xff, 0];
pub const CYAN: Rgb = [0, 0xff, 0xff];
pub const MAGENTA: Rgb = [0xff, 0, 0xff];

#[inline]
pub fn hex(s: &str) -> Rgb {
    let s = s.trim_start_matches('#');
    let bytes = s.as_bytes();
    let h = |i| {
        let hi = (bytes[i] as char).to_digit(16).unwrap_or(0);
        let lo = (bytes[i + 1] as char).to_digit(16).unwrap_or(0);
        ((hi << 4) | lo) as u8
    };
    [h(0), h(2), h(4)]
}

#[inline]
pub fn hsv(h: f32, s: f32, v: f32) -> Rgb {
    let h = h.rem_euclid(360.0);
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    [
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    ]
}

#[inline]
pub fn wheel(pos: u32) -> Rgb {
    let pos = (pos % 256) as i32;
    if pos < 85 {
        [(pos * 3) as u8, (255 - pos * 3) as u8, 0]
    } else if pos < 170 {
        let p = pos - 85;
        [(255 - p * 3) as u8, 0, (p * 3) as u8]
    } else {
        let p = pos - 170;
        [0, (p * 3) as u8, (255 - p * 3) as u8]
    }
}

#[inline]
pub fn interpolate(c1: Rgb, c2: Rgb, t: f32) -> Rgb {
    let t = t.clamp(0.0, 1.0);
    [
        (c1[0] as f32 + (c2[0] as f32 - c1[0] as f32) * t) as u8,
        (c1[1] as f32 + (c2[1] as f32 - c1[1] as f32) * t) as u8,
        (c1[2] as f32 + (c2[2] as f32 - c1[2] as f32) * t) as u8,
    ]
}

#[inline]
pub fn random_color() -> Rgb {
    let mut r = rand::thread_rng();
    [r.gen(), r.gen(), r.gen()]
}

#[inline]
pub fn scale(c: Rgb, brightness: f32) -> Rgb {
    let b = brightness.clamp(0.0, 1.0);
    [
        (c[0] as f32 * b) as u8,
        (c[1] as f32 * b) as u8,
        (c[2] as f32 * b) as u8,
    ]
}

pub fn gradient(colors: &[Rgb], steps: usize) -> Vec<Rgb> {
    if colors.len() < 2 {
        return vec![*colors.first().unwrap_or(&BLACK); steps];
    }
    let mut out = Vec::with_capacity(steps);
    let seg = steps / (colors.len() - 1);
    for i in 0..colors.len() - 1 {
        for j in 0..seg {
            let t = j as f32 / seg as f32;
            out.push(interpolate(colors[i], colors[i + 1], t));
        }
    }
    while out.len() < steps {
        out.push(*colors.last().unwrap());
    }
    out.truncate(steps);
    out
}

pub mod palette {
    use super::Rgb;
    pub const RAINBOW: &[Rgb] = &[
        [0xff, 0x00, 0x00],
        [0xff, 0x7f, 0x00],
        [0xff, 0xff, 0x00],
        [0x00, 0xff, 0x00],
        [0x00, 0x00, 0xff],
        [0x4b, 0x00, 0x82],
        [0x94, 0x00, 0xd3],
    ];
    pub const FIRE: &[Rgb] = &[
        [0xff, 0x00, 0x00],
        [0xff, 0x45, 0x00],
        [0xff, 0x8c, 0x00],
        [0xff, 0xd7, 0x00],
        [0xff, 0xff, 0x00],
    ];
    pub const OCEAN: &[Rgb] = &[
        [0x00, 0x00, 0x80],
        [0x00, 0x00, 0xff],
        [0x00, 0xbf, 0xff],
        [0x00, 0xff, 0xff],
        [0x40, 0xe0, 0xd0],
    ];
    pub const SUNSET: &[Rgb] = &[
        [0xff, 0x00, 0x00],
        [0xff, 0x45, 0x00],
        [0xff, 0x63, 0x47],
        [0xff, 0x7f, 0x50],
        [0xff, 0xa5, 0x00],
        [0xff, 0xd7, 0x00],
    ];
    pub const FOREST: &[Rgb] = &[
        [0x00, 0x64, 0x00],
        [0x22, 0x8b, 0x22],
        [0x32, 0xcd, 0x32],
        [0x00, 0xff, 0x00],
        [0x7f, 0xff, 0x00],
    ];
    pub const COOL: &[Rgb] = &[
        [0x00, 0x00, 0xff],
        [0x00, 0xff, 0xff],
        [0x00, 0xff, 0x00],
        [0xff, 0xff, 0x00],
    ];
    pub const WARM: &[Rgb] = &[
        [0xff, 0x00, 0x00],
        [0xff, 0x45, 0x00],
        [0xff, 0x8c, 0x00],
        [0xff, 0xff, 0x00],
    ];
    pub const NEON: &[Rgb] = &[
        [0xff, 0x00, 0xff],
        [0x00, 0xff, 0xff],
        [0x00, 0xff, 0x00],
        [0xff, 0xff, 0x00],
        [0xff, 0x00, 0x00],
    ];
    pub const ICE: &[Rgb] = &[
        [0xe0, 0xff, 0xff],
        [0xaf, 0xee, 0xee],
        [0x87, 0xce, 0xeb],
        [0x46, 0x82, 0xb4],
        [0x00, 0x00, 0xff],
    ];
}
