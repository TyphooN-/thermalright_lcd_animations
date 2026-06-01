# Animation Catalog — 79 Curated Animations

The catalog was tightened for aesthetics: harsh utility/status effects were removed from the public rotation set, several legacy effects were softened, and eight cinematic ambient animations were added.

## Retired from the public catalog

These were technically functional but not good-looking enough for daily rotation on the LCD: `police_strobe`, `emergency_strobe`, `morse_sos`, `strobe_multicolor`, `binary_counter`, `loading_bar`, `boot_sequence`, `traffic_light`, and `disco`.

## Added cinematic ambient set

1. **nebula_drift** — slow purple/blue nebula clouds; long dwell recommended.
2. **prism_bloom** — soft overlapping RGB blooms instead of hard flashes.
3. **ember_drift** — warm ember motes floating through smoky orange glow.
4. **ice_crystals** — icy facets with occasional silver shimmer.
5. **solar_flare** — twin golden flare sources with moving arcs.
6. **moonlit_tide** — deep blue tide with bright foam highlights.
7. **cyber_pulse** — teal/violet data pulses over a dim grid.
8. **jewel_box** — gemstone facets rotating over a dark base.

## Improved legacy effects

- **fire_wave** now uses layered coherent flame motion with restrained ember noise instead of per-pixel harsh random flicker.
- **random_burst** now creates soft radial blooms over a low ambient background instead of abrupt random full-bright bursts.
- **sparkle** is less noisy and longer-lived.
- **color_shift** now has stable, softened bands instead of choosing random band widths every frame.
- **ping_pong** and **mirror_bounce** now have trails, making them look intentional instead of like single diagnostic dots.

## Variable rotation timing

Yes: variable animation length is the right default. A flat 10 seconds cuts off slow-build ambient effects too early and leaves simple scanners/status loops on screen too long.

Runtime behavior:

```bash
# Headless auto-rotate: per-animation recommended dwell times
./target/release/thermalright-lcd --auto-rotate

# Force old flat timing
./target/release/thermalright-lcd --auto-rotate --duration 10 --fixed-duration
```

Config:

```json
{
  "rotation_duration": 10.0,
  "variable_rotation": true
}
```

When `variable_rotation` is true, `rotation_duration` is only the fixed-mode fallback; animation implementations can provide their own recommended dwell time.

## Current catalog

### Waves

- **rainbow_wave_ltr** — Rainbow wave moving left to right
- **rainbow_wave_rtl** — Rainbow wave moving right to left
- **dual_wave** — Dual rainbow waves on CPU and GPU sides
- **ocean_wave** — Ocean-themed wave with blue/cyan colors
- **fire_wave** — Smoother layered fire wave

### Scanners & chase

- **knight_rider** — Classic K.I.T.T. scanner
- **cylon_eye** — Battlestar-style scanner on both sides
- **larson_scanner_dual** — Dual colored scanners
- **chasing_lights** — Multi-color chase sequence
- **theater_chase** — Marquee theater chase
- **lighthouse** — Rotating warm beacon

### Patterns, color and gradients

- **checkerboard** — Animated checkerboard pattern
- **alternating_bars** — Moving color bars
- **spiral** — Spiral mathematical pattern
- **kaleidoscope** — Symmetric mirrored pattern
- **color_breathing** — Smooth pulsing glow
- **rainbow_cycle** — Full rainbow color cycle
- **rainbow_segments** — Each LCD region gets a different hue
- **gradient_sweep** — Gradient sweeping across display
- **sunset** — Sunset color transition
- **plasma** — Mathematical plasma effect
- **rainbow_spiral** — Rotating rainbow spiral
- **beat_pulse** — Synchronized beat pulse
- **color_shift** — Soft shifting color bands

### Nature and ambient

- **aurora** — Aurora borealis
- **lightning** — Lightning strike with afterglow
- **fireflies** — Twinkling fireflies
- **lava_lamp** — Lava lamp blobs
- **waterfall** — Cascading waterfall
- **nebula_drift** — Slow cinematic nebula clouds
- **prism_bloom** — Soft prismatic blooms
- **ember_drift** — Smoky warm ember drift
- **ice_crystals** — Crystalline ice shimmer
- **solar_flare** — Golden flare arcs
- **moonlit_tide** — Blue tide with silver foam
- **jewel_box** — Gemstone facets

### Particles & physics

- **sparkle** — Random sparkle/twinkle
- **random_burst** — Soft random color blooms
- **matrix_rain** — Matrix-style green rain
- **comet** — Comet with long trailing tail
- **fireworks** — Explosion bursts
- **meteor** — Meteor shower with trails
- **snake** — Snake crawling through LEDs
- **bouncing_ball** — Bouncing ball with physics
- **ping_pong** — Traced ball bouncing between CPU/GPU sides
- **bubbles** — Rising bubbles
- **confetti** — Confetti burst
- **random_walk** — Random-walk particles
- **pulse_ring** — Expanding pulse rings

### Audio-inspired

- **vu_meter** — VU meter bars
- **equalizer** — Multi-band equalizer
- **drum_circle** — Polyrhythmic pulses

### Retro & games

- **pacman** — Pac-man chasing a colored ghost
- **tetris_blocks** — Falling Tetris blocks

### Display-like but still visual

- **segment_crawl** — Crawl through all segments
- **color_wipe** — Color wipe on/off
- **scan_line** — Scanning line with trail
- **dna_helix** — DNA double helix pattern
- **binary_rain** — Binary rain columns
- **cyber_pulse** — Sleek data pulses

### Stars & space

- **stars** — Twinkling stars
- **warp_speed** — Star Trek warp effect
- **starfield_warp** — 3D starfield warp
- **wormhole** — Tunneling through colored rings

### Advanced / scientific effects

- **mirror_bounce** — Mirrored trails between sides
- **ripple** — Water ripple from two sources
- **rgb_windmills** — Rotating RGB windmills
- **glitch** — Glitch/corruption effect
- **scanner_sweep** — Multiple scanner beams
- **perlin_field** — Smooth noise color field
- **fluid_swirl** — Two-vortex fluid color advection
- **game_of_life** — 1D cellular automaton
- **ferrofluid** — Magnetic ferrofluid spikes
- **color_volcano** — Eruption of colored particles
- **double_pendulum** — Chaotic double-pendulum trace
- **interference** — Two-source wave interference
- **magnetic_field** — Dipole field between two poles
- **predator_prey** — Lotka-Volterra population dynamics

## Tips

- Use variable rotation for unattended mode. Slow ambient animations need 14–18 seconds; scanners and simple loops are fine around 10 seconds.
- Keep frame interval near 0.015s unless the hardware link is stable at faster rates.
- If you want a calmer daily rotation, prioritize: `nebula_drift`, `prism_bloom`, `ember_drift`, `ice_crystals`, `moonlit_tide`, `aurora`, `lava_lamp`, `fluid_swirl`, `perlin_field`, `stars`.
