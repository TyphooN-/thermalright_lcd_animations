mod animations;
mod color;
mod interactive;
mod lcd;
mod led_map;

use std::path::PathBuf;
use std::thread::sleep;
use std::time::{Duration, Instant};

use clap::Parser;
use serde::Deserialize;

use crate::interactive::InteractiveController;
use crate::lcd::LcdController;

const DEFAULT_UPDATE_INTERVAL: f32 = 0.015;
const DEFAULT_ROTATION_DURATION: f32 = 10.0;
const DEFAULT_VENDOR_ID: u16 = 0x0416;
const DEFAULT_PRODUCT_ID: u16 = 0x8001;

#[derive(Debug, Deserialize, Default)]
struct Config {
    vendor_id: Option<String>,
    product_id: Option<String>,
    animation_mode: Option<String>,
    rotation_duration: Option<f32>,
    variable_rotation: Option<bool>,
    update_interval: Option<f32>,
    animations: Option<Vec<String>>,
}

fn parse_hex_u16(s: &str) -> u16 {
    let s = s.trim_start_matches("0x").trim_start_matches("0X");
    u16::from_str_radix(s, 16).unwrap_or(0)
}

#[derive(Parser, Debug)]
#[command(
    name = "thermalright-lcd",
    version,
    about = "Thermalright LCD Animations (Rust)"
)]
struct Cli {
    /// Path to config file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Run a single animation by name
    #[arg(short, long)]
    animation: Option<String>,

    /// List all available animations
    #[arg(short, long)]
    list: bool,

    /// Auto-rotate duration per animation, in seconds
    #[arg(short, long)]
    duration: Option<f32>,

    /// Update interval between frames, in seconds (default 0.015)
    #[arg(short, long)]
    interval: Option<f32>,

    /// Interactive mode with keyboard controls
    #[arg(long)]
    interactive: bool,

    /// Headless auto-rotate mode
    #[arg(long)]
    auto_rotate: bool,

    /// Disable per-animation recommended dwell times in auto-rotate mode
    #[arg(long)]
    fixed_duration: bool,
}

fn load_config(path: Option<&PathBuf>) -> Config {
    let default_path = PathBuf::from("config.json");
    let resolved = path.cloned().unwrap_or_else(|| {
        // Try ../config.json relative to binary dir, otherwise cwd
        if default_path.exists() {
            default_path.clone()
        } else {
            let mut p = std::env::current_exe().unwrap_or_default();
            p.pop();
            p.pop();
            p.push("config.json");
            if p.exists() {
                p
            } else {
                default_path
            }
        }
    });
    std::fs::read_to_string(&resolved)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn list_animations() {
    let entries = animations::all();
    println!("\n Available Animations ({})", entries.len());
    println!("======================================================================");
    let mut sorted: Vec<_> = entries.iter().collect();
    sorted.sort_by_key(|e| e.name);
    for e in &sorted {
        println!("  {:<22} - {}", e.name, e.desc);
    }
    println!("======================================================================");
}

fn run_single(lcd: &mut LcdController, name: &str, interval: f32) -> i32 {
    let entries = animations::all();
    let entry = match entries.iter().find(|e| e.name == name) {
        Some(e) => e,
        None => {
            eprintln!("Error: Animation '{}' not found.", name);
            list_animations();
            return 1;
        }
    };
    println!("\nRunning animation: {}", entry.name);
    println!("Press Ctrl+C to stop\n");
    let mut anim = (entry.factory)();
    anim.reset(lcd);
    loop {
        anim.update(lcd);
        lcd.send_packets();
        sleep(Duration::from_secs_f32(interval));
    }
}

fn run_auto_rotate(
    lcd: &mut LcdController,
    selected: &[String],
    rotation_duration: f32,
    variable_rotation: bool,
    interval: f32,
) -> i32 {
    let entries = animations::all();
    println!("\n Auto-Rotate Mode");
    println!("======================================================================");
    println!(
        "Rotation Duration: {}",
        if variable_rotation {
            "variable per animation".to_string()
        } else {
            format!("{}s fixed", rotation_duration)
        }
    );
    println!("Update Interval: {}s", interval);
    println!("Animations: {}", selected.len());
    println!("\nPress Ctrl+C to stop\n");

    let mut idx = 0;
    loop {
        let name = &selected[idx];
        let entry = match entries.iter().find(|e| e.name == name) {
            Some(e) => e,
            None => {
                eprintln!("Warning: animation '{}' not found, skipping", name);
                idx = (idx + 1) % selected.len();
                continue;
            }
        };
        println!("[{}/{}] {}", idx + 1, selected.len(), entry.name);
        let mut anim = (entry.factory)();
        let effective_duration = if variable_rotation {
            anim.preferred_duration()
        } else {
            rotation_duration
        };
        println!("    dwell: {:.1}s", effective_duration);
        anim.reset(lcd);
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs_f32(effective_duration) {
            anim.update(lcd);
            lcd.send_packets();
            sleep(Duration::from_secs_f32(interval));
        }
        idx = (idx + 1) % selected.len();
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.list {
        list_animations();
        return;
    }

    let config = load_config(cli.config.as_ref());

    let vendor_id = config
        .vendor_id
        .as_deref()
        .map(parse_hex_u16)
        .filter(|&v| v != 0)
        .unwrap_or(DEFAULT_VENDOR_ID);
    let product_id = config
        .product_id
        .as_deref()
        .map(parse_hex_u16)
        .filter(|&v| v != 0)
        .unwrap_or(DEFAULT_PRODUCT_ID);
    let interval = cli
        .interval
        .or(config.update_interval)
        .unwrap_or(DEFAULT_UPDATE_INTERVAL);
    let duration = cli
        .duration
        .or(config.rotation_duration)
        .unwrap_or(DEFAULT_ROTATION_DURATION);
    let variable_rotation = config.variable_rotation.unwrap_or(true) && !cli.fixed_duration;

    println!("Initializing LCD Controller...");
    println!("Vendor ID: 0x{:04x}", vendor_id);
    println!("Product ID: 0x{:04x}", product_id);

    let mut lcd = match LcdController::new(vendor_id, product_id) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("HID init error: {}", e);
            std::process::exit(1);
        }
    };

    if !lcd.is_connected() {
        eprintln!("\nError: Could not connect to LCD device.");
        eprintln!("Make sure:");
        eprintln!("  1. The device is connected");
        eprintln!("  2. You have proper permissions (run sudo ./install.sh)");
        eprintln!("  3. The vendor/product IDs are correct");
        std::process::exit(1);
    }

    println!("Connected successfully!\n");

    if let Some(name) = cli.animation {
        std::process::exit(run_single(&mut lcd, &name, interval));
    }

    let mode = config.animation_mode.as_deref().unwrap_or("interactive");
    if !cli.auto_rotate && (cli.interactive || mode == "interactive") {
        let mut controller = InteractiveController::new(lcd, interval);
        if let Err(e) = controller.run() {
            eprintln!("Interactive error: {}", e);
            std::process::exit(1);
        }
        return;
    }

    let selected: Vec<String> = config.animations.unwrap_or_else(|| {
        animations::all()
            .iter()
            .map(|e| e.name.to_string())
            .collect()
    });
    std::process::exit(run_auto_rotate(
        &mut lcd,
        &selected,
        duration,
        variable_rotation,
        interval,
    ));
}
