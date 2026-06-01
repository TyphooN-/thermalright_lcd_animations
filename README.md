# Thermalright LCD Animations

A creative animation suite for Thermalright Peerless Assassin USB LCD coolers, written in Rust. Drives all 84 LEDs across both CPU and GPU panels with 79 curated animations.

This repo started as a Python project; it was rewritten in Rust to slash its 24/7 footprint. Measured on the same hardware running the same animation at the same frame rate:

| | Python | Rust | Δ |
|---|---|---|---|
| Resident memory | 30.5 MB | 3.8 MB | **-88%** |
| CPU (one core) | ~1.0% | 0.2% | **-5x** |
| Process layout | interpreter + numpy + hid | single static binary | — |
| Animations | 68 | 79 curated | +11 net after retiring harsh/status effects |

The bottleneck is USB HID I/O, not host CPU — but eliminating Python's interpreter overhead recovers ~27 MB of RAM and a measurable fraction of a core that would otherwise be paid every second for the life of the system.

## Features

- 79 curated animations across waves, scanners, patterns, effects, physics, nature, retro games, cinematic ambient loops, and chaotic/scientific simulations
- Interactive TUI with keyboard navigation (`←`/`→`/space/m/`±`/`[`/`]`/q)
- Single-animation mode and auto-rotate mode
- Configurable via CLI flags or `config.json`
- Optional systemd service for autostart
- Single static binary, ~1 MB, no runtime dependencies

## Hardware

- Peerless Assassin 120 Digital
- Peerless Assassin 140 Digital
- Other Thermalright USB LCDs that speak the same HID protocol (VID `0x0416`, PID `0x8001`)

## Build

Requires a Rust toolchain (1.70+). Install via [rustup](https://rustup.rs/) if you don't have one.

```bash
cargo build --release
```

The binary lands at `./target/release/thermalright-lcd`.

## Install (optional)

Sets up a `udev` rule so the device is accessible without root, builds the binary if needed, and optionally registers a systemd service:

```bash
sudo ./install.sh
```

## Usage

```bash
# Interactive TUI (default — drops you straight into it)
./target/release/thermalright-lcd

# List all available animations
./target/release/thermalright-lcd --list

# Run one animation continuously
./target/release/thermalright-lcd --animation game_of_life

# Headless auto-rotate through every animation using per-animation dwell times by default
./target/release/thermalright-lcd --auto-rotate

# Force old fixed-duration rotation
./target/release/thermalright-lcd --auto-rotate --duration 10 --fixed-duration

# Faster frame interval (default 0.015s = ~67 fps)
./target/release/thermalright-lcd --interval 0.01
```

### Interactive controls

| Key | Action |
|---|---|
| `←` / `→` | Previous / Next animation |
| `space` | Jump to a random animation |
| `m` | Toggle Manual / Random rotation mode |
| `+` / `-` | Increase / Decrease rotation duration (±1 s) |
| `[` / `]` | Slower / Faster frame interval (±5 ms) |
| `q` or `Ctrl+C` | Quit |

## Configuration

`config.json` is optional. CLI flags override its values. Recognized keys:

```json
{
  "vendor_id": "0x0416",
  "product_id": "0x8001",
  "animation_mode": "interactive",
  "rotation_duration": 10.0,
  "variable_rotation": true,
  "update_interval": 0.015,
  "animations": ["rainbow_cycle", "game_of_life", "fluid_swirl", "..."]
}
```

If `animation_mode` is `"interactive"` (or missing) the TUI is the default. Set it to anything else (e.g. `"auto_rotate"`) to run headless rotation. With `variable_rotation: true` (default), each animation uses its own recommended dwell time; pass `--fixed-duration` or set `variable_rotation: false` to force `rotation_duration` for every animation.

## Animation catalog

See [`ANIMATIONS.md`](ANIMATIONS.md) for the full curated list, including the Rust-only simulations plus the new cinematic ambient set (Nebula Drift, Prism Bloom, Ember Drift, Ice Crystals, Solar Flare, Moonlit Tide, Cyber Pulse, Jewel Box). Harsh/status effects such as police/emergency strobes, SOS, loading bars, binary counter, traffic light, and disco were retired from the public catalog.

## License

MIT — see [`LICENSE`](LICENSE).
