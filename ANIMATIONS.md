# Animation Catalog — 80 Animations

68 ported from the original Python implementation plus 12 new Rust-only additions, organized by category.

## Waves (5)

1. **rainbow_wave_ltr** — Rainbow wave moving left to right
2. **rainbow_wave_rtl** — Rainbow wave moving right to left
3. **dual_wave** — Dual rainbow waves on CPU and GPU sides
4. **ocean_wave** — Ocean-themed wave with blue/cyan colors
5. **fire_wave** — Flickering fire wave with red/orange/yellow

## Scanners & chase (6)

6. **knight_rider** — Classic K.I.T.T. scanner (red bouncing)
7. **cylon_eye** — Battlestar Galactica scanner (both sides)
8. **larson_scanner_dual** — Dual colored scanners (cyan/magenta)
9. **chasing_lights** — Multi-color chase sequence
10. **theater_chase** — Marquee theater chase
11. **lighthouse** — Rotating lighthouse beacon

## Patterns (5)

12. **police_strobe** — Red/blue police strobe
13. **checkerboard** — Animated checkerboard pattern
14. **alternating_bars** — Color bar patterns
15. **spiral** — Spiral mathematical pattern
16. **kaleidoscope** — Symmetric kaleidoscope pattern

## Pulse & strobe (5)

17. **color_breathing** — Smooth pulsing glow with color cycle
18. **heartbeat** — Two-beat heartbeat pulse
19. **emergency_strobe** — Emergency vehicle strobe
20. **morse_sos** — Morse code SOS (... --- ...)
21. **strobe_multicolor** — Random multi-color strobes

## Color cycle & gradient (8)

22. **rainbow_cycle** — Full rainbow color cycle
23. **rainbow_segments** — Each segment region gets a different hue
24. **gradient_sweep** — Gradient sweeping across display
25. **sunset** — Sunset color transition (blue→orange→purple→black)
26. **plasma** — Mathematical plasma effect
27. **rainbow_spiral** — Spiral rainbow rotation
28. **beat_pulse** — Synchronized beat pulse
29. **color_shift** — Shifting color bands

## Nature (5)

30. **aurora** — Aurora borealis
31. **lightning** — Lightning strike with afterglow
32. **fireflies** — Twinkling fireflies
33. **lava_lamp** — Lava lamp blobs
34. **waterfall** — Cascading waterfall

## Particles & physics (10)

35. **sparkle** — Random sparkle/twinkle
36. **random_burst** — Random color bursts
37. **matrix_rain** — Matrix-style green rain
38. **comet** — Comet with long trailing tail
39. **fireworks** — Explosion bursts
40. **meteor** — Meteor shower with trails
41. **snake** — Snake crawling through LEDs
42. **bouncing_ball** — Bouncing ball with physics
43. **ping_pong** — Ball bouncing between CPU/GPU sides
44. **bubbles** — Rising bubbles

## Audio-inspired (3)

45. **vu_meter** — VU meter bars (per side)
46. **equalizer** — Multi-band equalizer
47. **drum_circle** — Polyrhythmic pulses (NEW)

## Retro & games (2)

48. **pacman** — Pac-man chasing a colored ghost
49. **tetris_blocks** — Falling Tetris blocks

## Display & system (8)

50. **binary_counter** — Binary number counter
51. **segment_crawl** — Crawl through all segments
52. **loading_bar** — Progress loading bar
53. **color_wipe** — Color wipe on/off
54. **boot_sequence** — System boot sequence
55. **scan_line** — Scanning line with trail
56. **traffic_light** — Red → yellow → green cycle
57. **dna_helix** — DNA double helix pattern

## Stars & space (4)

58. **stars** — Twinkling stars
59. **disco** — Disco lights with random flash
60. **warp_speed** — Star Trek warp effect
61. **binary_rain** — Binary rain columns

## Advanced effects (8)

62. **mirror_bounce** — Mirrored bouncing between sides
63. **pulse_ring** — Expanding pulse rings from center
64. **random_walk** — Random walk particles
65. **glitch** — Glitch / corruption effect
66. **scanner_sweep** — Multiple scanner beams
67. **confetti** — Confetti bursts
68. **ripple** — Water ripple from two sources
69. **rgb_windmills** — Rotating RGB windmills

## New in Rust (12) ⚡

70. **perlin_field** — Smooth perlin-like noise color field
71. **fluid_swirl** — Two-vortex fluid color advection
72. **game_of_life** — 1D cellular automaton (rule 110)
73. **ferrofluid** — Magnetic ferrofluid spikes with falloff
74. **color_volcano** — Particles erupting from center
75. **double_pendulum** — Chaotic double pendulum trace
76. **wormhole** — Tunneling through concentric colored rings
77. **starfield_warp** — 3D starfield warp (depth + streaks)
78. **drum_circle** — Polyrhythmic pulses with coprime periods
79. **interference** — Two-source wave interference
80. **magnetic_field** — Dipole field between two poles
81. **predator_prey** — Lotka-Volterra population dynamics

## Usage

```bash
# Auto-rotate through every animation, 10s each
./target/release/thermalright-lcd --duration 10

# Run a specific animation continuously
./target/release/thermalright-lcd --animation game_of_life

# Faster frame rate
./target/release/thermalright-lcd --interval 0.01
```

## Tips

- Frame interval 0.015s (~67 fps) is the default; many animations look great at 0.01s
- Nature animations (aurora, lightning, fireflies) are subtle and atmospheric
- New chaotic ones (double_pendulum, game_of_life, predator_prey) never look the same twice
- Scanner classics (knight_rider, cylon_eye) are still hard to beat
