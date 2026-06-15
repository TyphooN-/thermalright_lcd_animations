use crate::lcd::LcdController;

pub trait Animation: Send {
    fn update(&mut self, lcd: &mut LcdController);

    fn reset(&mut self, lcd: &mut LcdController) {
        lcd.clear();
    }

    /// Recommended auto-rotation dwell time for this animation.
    ///
    /// Effects with slow build-up or narrative arcs need longer than quick
    /// scanners/strobes. The runner can still force a fixed duration.
    fn preferred_duration(&self) -> f32 {
        10.0
    }
}

mod base;
mod extended;
mod extras;

pub type Factory = fn() -> Box<dyn Animation>;

pub struct Entry {
    pub name: &'static str,
    pub desc: &'static str,
    pub factory: Factory,
}

pub fn all() -> &'static [Entry] {
    REGISTRY
}

#[rustfmt::skip]
const REGISTRY: &[Entry] = &[
    // ---- base library (from animation_library.py) ----
    Entry { name: "rainbow_wave_ltr",   desc: "Rainbow wave moving left to right",      factory: || Box::new(base::RainbowWaveLtr::default()) },
    Entry { name: "rainbow_wave_rtl",   desc: "Rainbow wave moving right to left",      factory: || Box::new(base::RainbowWaveRtl::default()) },
    Entry { name: "dual_wave",          desc: "Dual rainbow waves on CPU and GPU",      factory: || Box::new(base::DualWave::default()) },
    Entry { name: "ocean_wave",         desc: "Ocean-themed wave",                       factory: || Box::new(base::OceanWave::default()) },
    Entry { name: "fire_wave",          desc: "Flickering fire wave",                    factory: || Box::new(base::FireWave::default()) },
    Entry { name: "knight_rider",       desc: "K.I.T.T. scanner effect",                 factory: || Box::new(base::KnightRider::default()) },
    Entry { name: "cylon_eye",          desc: "Cylon eye scanner (both sides)",          factory: || Box::new(base::CylonEye::default()) },
    Entry { name: "larson_scanner_dual",desc: "Dual Larson scanners on CPU and GPU",     factory: || Box::new(base::LarsonScannerDual::default()) },
    Entry { name: "chasing_lights",     desc: "Chasing lights",                          factory: || Box::new(base::ChasingLights::default()) },
    Entry { name: "theater_chase",      desc: "Theater marquee chase",                   factory: || Box::new(base::TheaterChase::default()) },
    Entry { name: "checkerboard",       desc: "Checkerboard pattern",                    factory: || Box::new(base::Checkerboard::default()) },
    Entry { name: "alternating_bars",   desc: "Alternating color bars",                  factory: || Box::new(base::AlternatingBars::default()) },
    Entry { name: "color_breathing",    desc: "Smooth breathing effect",                 factory: || Box::new(base::ColorBreathing::default()) },
    Entry { name: "rainbow_cycle",      desc: "Rainbow color cycle",                     factory: || Box::new(base::RainbowCycle::default()) },
    Entry { name: "sparkle",            desc: "Random sparkle / twinkle",                factory: || Box::new(base::Sparkle::default()) },
    Entry { name: "random_burst",       desc: "Random color bursts",                     factory: || Box::new(base::RandomBurst::default()) },
    Entry { name: "gradient_sweep",     desc: "Gradient sweep across display",           factory: || Box::new(base::GradientSweep::new()) },
    Entry { name: "plasma",             desc: "Plasma effect",                           factory: || Box::new(base::Plasma::new()) },
    Entry { name: "matrix_rain",        desc: "Matrix-style rain",                       factory: || Box::new(base::MatrixRain::new()) },
    Entry { name: "segment_crawl",      desc: "Crawl through display segments",          factory: || Box::new(base::SegmentCrawl::default()) },
    Entry { name: "color_wipe",         desc: "Color wipe effect",                       factory: || Box::new(base::ColorWipe::default()) },
    Entry { name: "rainbow_segments",   desc: "Each region gets different rainbow color",factory: || Box::new(base::RainbowSegments::default()) },

    // ---- extended library (from animation_library_extended.py) ----
    Entry { name: "heartbeat",          desc: "Heartbeat pulse",                         factory: || Box::new(extended::Heartbeat::default()) },
    Entry { name: "lighthouse",         desc: "Rotating lighthouse beacon",              factory: || Box::new(extended::Lighthouse::default()) },
    Entry { name: "snake",              desc: "Snake crawling",                          factory: || Box::new(extended::Snake::default()) },
    Entry { name: "bouncing_ball",      desc: "Bouncing ball with physics",              factory: || Box::new(extended::BouncingBall::new()) },
    Entry { name: "ping_pong",          desc: "Ball ping pong between CPU and GPU",      factory: || Box::new(extended::PingPong::new()) },
    Entry { name: "spiral",             desc: "Spiral pattern",                          factory: || Box::new(extended::Spiral::default()) },
    Entry { name: "vu_meter",           desc: "VU meter simulation",                     factory: || Box::new(extended::VuMeter::default()) },
    Entry { name: "equalizer",          desc: "Equalizer bars",                          factory: || Box::new(extended::Equalizer::new()) },
    Entry { name: "beat_pulse",         desc: "Beat pulse",                              factory: || Box::new(extended::BeatPulse::default()) },
    Entry { name: "lightning",          desc: "Lightning strike effect",                 factory: || Box::new(extended::Lightning::new()) },
    Entry { name: "aurora",             desc: "Aurora borealis",                         factory: || Box::new(extended::Aurora::default()) },
    Entry { name: "fireflies",          desc: "Fireflies twinkling",                     factory: || Box::new(extended::Fireflies::default()) },
    Entry { name: "lava_lamp",          desc: "Lava lamp",                               factory: || Box::new(extended::LavaLamp::default()) },
    Entry { name: "pacman",             desc: "Pac-man chase",                           factory: || Box::new(extended::Pacman::new()) },
    Entry { name: "tetris_blocks",      desc: "Falling Tetris blocks",                   factory: || Box::new(extended::TetrisBlocks::default()) },
    Entry { name: "comet",              desc: "Comet with long tail",                    factory: || Box::new(extended::Comet::default()) },
    Entry { name: "fireworks",          desc: "Fireworks explosions",                    factory: || Box::new(extended::Fireworks::default()) },
    Entry { name: "waterfall",          desc: "Waterfall cascade",                       factory: || Box::new(extended::Waterfall::default()) },
    Entry { name: "dna_helix",          desc: "DNA double helix",                        factory: || Box::new(extended::DnaHelix::default()) },
    Entry { name: "rainbow_spiral",     desc: "Rainbow spiral",                          factory: || Box::new(extended::RainbowSpiral::default()) },
    Entry { name: "mirror_bounce",      desc: "Mirror effect bouncing between sides",    factory: || Box::new(extended::MirrorBounce::default()) },
    Entry { name: "sunset",             desc: "Sunset color transition",                 factory: || Box::new(extended::Sunset::default()) },
    Entry { name: "scan_line",          desc: "Scanning line",                           factory: || Box::new(extended::ScanLine::default()) },
    Entry { name: "kaleidoscope",       desc: "Kaleidoscope",                            factory: || Box::new(extended::Kaleidoscope::default()) },
    Entry { name: "meteor",             desc: "Meteor shower",                           factory: || Box::new(extended::Meteor::default()) },
    Entry { name: "rgb_windmills",      desc: "Rotating RGB windmills",                  factory: || Box::new(extended::RgbWindmills::default()) },
    Entry { name: "bubbles",            desc: "Rising bubbles",                          factory: || Box::new(extended::Bubbles::default()) },
    Entry { name: "stars",              desc: "Twinkling stars",                         factory: || Box::new(extended::Stars::new()) },
    Entry { name: "warp_speed",         desc: "Star Trek warp speed",                    factory: || Box::new(extended::WarpSpeed::default()) },
    Entry { name: "binary_rain",        desc: "Binary rain",                             factory: || Box::new(extended::BinaryRain::new()) },
    Entry { name: "pulse_ring",         desc: "Expanding pulse rings",                   factory: || Box::new(extended::PulseRing::default()) },
    Entry { name: "color_shift",        desc: "Shifting color bands",                    factory: || Box::new(extended::ColorShift::default()) },
    Entry { name: "random_walk",        desc: "Random walk particles",                   factory: || Box::new(extended::RandomWalk::new()) },
    Entry { name: "glitch",             desc: "Glitch / corruption effect",              factory: || Box::new(extended::Glitch::default()) },
    Entry { name: "scanner_sweep",      desc: "Multiple scanner beams",                  factory: || Box::new(extended::ScannerSweep::new()) },
    Entry { name: "confetti",           desc: "Confetti burst",                          factory: || Box::new(extended::Confetti::default()) },
    Entry { name: "ripple",             desc: "Water ripple",                            factory: || Box::new(extended::Ripple::default()) },

    // ---- new animations (Rust-only additions) ----
    Entry { name: "perlin_field",       desc: "Smooth perlin-like noise color field",    factory: || Box::new(extras::PerlinField::default()) },
    Entry { name: "fluid_swirl",        desc: "Two-vortex fluid color advection",        factory: || Box::new(extras::FluidSwirl::default()) },
    Entry { name: "game_of_life",       desc: "1D cellular automaton (rule 110-ish)",    factory: || Box::new(extras::GameOfLife::new()) },
    Entry { name: "ferrofluid",         desc: "Magnetic ferrofluid spikes",              factory: || Box::new(extras::Ferrofluid::default()) },
    Entry { name: "color_volcano",      desc: "Eruption of colored particles",           factory: || Box::new(extras::ColorVolcano::default()) },
    Entry { name: "double_pendulum",    desc: "Chaotic double pendulum trace",           factory: || Box::new(extras::DoublePendulum::new()) },
    Entry { name: "wormhole",           desc: "Tunneling through colored rings",         factory: || Box::new(extras::Wormhole::default()) },
    Entry { name: "starfield_warp",     desc: "3D starfield warp (depth sorted)",        factory: || Box::new(extras::StarfieldWarp::new()) },
    Entry { name: "drum_circle",        desc: "Polyrhythmic pulses bouncing",            factory: || Box::new(extras::DrumCircle::new()) },
    Entry { name: "interference",       desc: "Two-source wave interference",            factory: || Box::new(extras::Interference::default()) },
    Entry { name: "magnetic_field",     desc: "Field lines between two poles",           factory: || Box::new(extras::MagneticField::default()) },
    Entry { name: "predator_prey",      desc: "Lotka-Volterra population color swap",    factory: || Box::new(extras::PredatorPrey::new()) },
    Entry { name: "nebula_drift",       desc: "Slow cinematic nebula clouds",            factory: || Box::new(extras::NebulaDrift::default()) },
    Entry { name: "prism_bloom",        desc: "Soft prismatic blooms and fades",          factory: || Box::new(extras::PrismBloom::default()) },
    Entry { name: "ember_drift",        desc: "Warm embers floating on smoky glow",       factory: || Box::new(extras::EmberDrift::default()) },
    Entry { name: "ice_crystals",       desc: "Crystalline ice facets and shimmer",      factory: || Box::new(extras::IceCrystals::default()) },
    Entry { name: "solar_flare",        desc: "Golden flare arcs from twin suns",         factory: || Box::new(extras::SolarFlare::default()) },
    Entry { name: "moonlit_tide",       desc: "Blue moonlit tide with silver foam",       factory: || Box::new(extras::MoonlitTide::default()) },
    Entry { name: "cyber_pulse",        desc: "Sleek teal/violet data pulses",            factory: || Box::new(extras::CyberPulse::default()) },
    Entry { name: "jewel_box",          desc: "Gemstone facets rotating in velvet dark",  factory: || Box::new(extras::JewelBox::default()) },
    Entry { name: "silk_drift",         desc: "Slow flowing silk-like pastel bands",   factory: || Box::new(extras::SilkDrift::default()) },
    Entry { name: "crystal_shimmer",    desc: "Icy crystal facets with soft shimmers",   factory: || Box::new(extras::CrystalShimmer::default()) },
    Entry { name: "void_pulse",         desc: "Deep space breathing rings",            factory: || Box::new(extras::VoidPulse::default()) },
    Entry { name: "pollen_drift",       desc: "Sparse warm pollen motes over cool field", factory: || Box::new(extras::PollenDrift::default()) },
    Entry { name: "tidepool_caustic",   desc: "Two slow caustic tide pools",           factory: || Box::new(extras::TidepoolCaustic::default()) },
];
