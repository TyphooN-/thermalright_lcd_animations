use std::io::{stdout, Write};
use std::time::{Duration, Instant};

use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{execute, queue};
use rand::Rng;

use crate::animations::{self, Animation};
use crate::lcd::LcdController;

pub struct InteractiveController {
    lcd: LcdController,
    entries: &'static [animations::Entry],
    current_index: usize,
    current_anim: Option<Box<dyn Animation>>,
    duration: f32,
    update_interval: f32,
    random_mode: bool,
    start_time: Instant,
}

impl InteractiveController {
    pub fn new(lcd: LcdController, update_interval: f32) -> Self {
        Self {
            lcd,
            entries: animations::all(),
            current_index: 0,
            current_anim: None,
            duration: 10.0,
            update_interval,
            random_mode: true,
            start_time: Instant::now(),
        }
    }

    fn load_current(&mut self) {
        let entry = &self.entries[self.current_index];
        let mut anim = (entry.factory)();
        anim.reset(&mut self.lcd);
        self.duration = anim.preferred_duration();
        self.current_anim = Some(anim);
        self.start_time = Instant::now();
        let _ = self.draw_ui();
    }

    fn next(&mut self) {
        self.current_index = (self.current_index + 1) % self.entries.len();
        self.load_current();
    }

    fn prev(&mut self) {
        self.current_index = if self.current_index == 0 {
            self.entries.len() - 1
        } else {
            self.current_index - 1
        };
        self.load_current();
    }

    fn random(&mut self) {
        let mut rng = rand::thread_rng();
        self.current_index = rng.gen_range(0..self.entries.len());
        self.load_current();
    }

    fn adjust_duration(&mut self, delta: f32) {
        self.duration = (self.duration + delta).max(1.0);
        let _ = self.draw_ui();
    }

    fn adjust_speed(&mut self, faster: bool) {
        if faster {
            self.update_interval = (self.update_interval - 0.005).max(0.001);
        } else {
            self.update_interval = (self.update_interval + 0.005).min(0.2);
        }
        let _ = self.draw_ui();
    }

    fn toggle_random(&mut self) {
        self.random_mode = !self.random_mode;
        let _ = self.draw_ui();
    }

    fn draw_ui(&self) -> std::io::Result<()> {
        let mut out = stdout();
        let entry = &self.entries[self.current_index];
        queue!(out, Clear(ClearType::All), MoveTo(0, 0))?;
        queue!(
            out,
            Print("======================================================================\r\n")
        )?;
        queue!(
            out,
            Print(" THERMALRIGHT LCD ANIMATIONS - Interactive Mode (Rust)\r\n")
        )?;
        queue!(
            out,
            Print("======================================================================\r\n")
        )?;
        queue!(out, Print("\r\n"))?;
        queue!(
            out,
            Print(format!(
                " Current Animation: [{}/{}]\r\n",
                self.current_index + 1,
                self.entries.len()
            ))
        )?;
        queue!(out, Print(format!(" {} - {}\r\n", entry.name, entry.desc)))?;
        queue!(out, Print("\r\n"))?;
        queue!(
            out,
            Print(format!(
                " Mode: {}\r\n",
                if self.random_mode { "RANDOM" } else { "MANUAL" }
            ))
        )?;
        queue!(
            out,
            Print(format!(
                " Dwell: {:.1}s (animation recommended)\r\n",
                self.duration
            ))
        )?;
        queue!(
            out,
            Print(format!(
                " Speed (interval): {:.3}s\r\n",
                self.update_interval
            ))
        )?;
        queue!(out, Print("\r\n"))?;
        queue!(
            out,
            Print("----------------------------------------------------------------------\r\n")
        )?;
        queue!(out, Print(" Controls:\r\n"))?;
        queue!(out, Print("   <- ->  : Previous/Next animation\r\n"))?;
        queue!(out, Print("   SPACE  : Jump to random animation\r\n"))?;
        queue!(out, Print("   m      : Toggle Manual/Random mode\r\n"))?;
        queue!(
            out,
            Print("   + -    : Increase/Decrease duration (+/- 1s)\r\n")
        )?;
        queue!(out, Print("   [ ]    : Decrease/Increase speed\r\n"))?;
        queue!(out, Print("   q      : Quit\r\n"))?;
        queue!(
            out,
            Print("----------------------------------------------------------------------\r\n")
        )?;
        out.flush()
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        enable_raw_mode()?;
        execute!(stdout(), EnterAlternateScreen, Hide)?;

        if self.random_mode {
            self.random();
        } else {
            self.load_current();
        }

        let result = self.event_loop();

        execute!(stdout(), Show, LeaveAlternateScreen)?;
        disable_raw_mode()?;

        self.lcd.clear();
        self.lcd.send_packets();

        result
    }

    fn event_loop(&mut self) -> std::io::Result<()> {
        loop {
            if event::poll(Duration::from_secs(0))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            break
                        }
                        KeyCode::Right => self.next(),
                        KeyCode::Left => self.prev(),
                        KeyCode::Char(' ') => self.random(),
                        KeyCode::Char('m') | KeyCode::Char('M') => self.toggle_random(),
                        KeyCode::Char('+') | KeyCode::Char('=') => self.adjust_duration(1.0),
                        KeyCode::Char('-') | KeyCode::Char('_') => self.adjust_duration(-1.0),
                        KeyCode::Char('[') => self.adjust_speed(false),
                        KeyCode::Char(']') => self.adjust_speed(true),
                        _ => {}
                    }
                }
            }

            if self.random_mode {
                if self.start_time.elapsed() >= Duration::from_secs_f32(self.duration) {
                    self.random();
                }
            }

            if let Some(anim) = self.current_anim.as_mut() {
                anim.update(&mut self.lcd);
                self.lcd.send_packets();
            }

            std::thread::sleep(Duration::from_secs_f32(self.update_interval));
        }
        Ok(())
    }
}
