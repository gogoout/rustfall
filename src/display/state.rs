use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};
use strum::IntoEnumIterator;

use crate::display::event::Event;
use crate::display::render::Renderer;
use crate::engine::pixel::Pixel;
use crate::engine::sandbox::Sandbox;

/// Application.
#[derive(Debug)]
pub struct State {
    /// should the application exit?
    pub should_quit: bool,
    pub sandbox: Sandbox,
    pub active_pixel: Pixel,
    no_braille: bool,
    mouse_down_event: Option<MouseEvent>,
    pause: bool,
}

impl State {
    /// Constructs a new instance of [`State`].
    pub fn new(width: usize, height: usize, no_braille: bool) -> Self {
        let (width, height) = Self::calculate_sandbox_size(width, height, no_braille);

        Self {
            should_quit: false,
            sandbox: Sandbox::new(width, height),
            active_pixel: Default::default(),
            no_braille,
            mouse_down_event: None,
            pause: false,
        }
    }

    fn calculate_sandbox_size(width: usize, height: usize, no_braille: bool) -> (usize, usize) {
        let (width, height) = Renderer::sandbox_size(width, height);
        match no_braille {
            true => (width, height),
            false => (width * 2, height * 4),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        self.handle_mouse_down_event();
        if !self.pause {
            self.sandbox.tick();
        }
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn update(&mut self, event: Event) {
        match event {
            Event::Tick => self.tick(),
            Event::Key(key) => self.handle_key_event(key),
            Event::Mouse(mouse) => {
                self.handle_mouse_event(mouse);
            }
            Event::Resize(width, height) => {
                let (width, height) =
                    Self::calculate_sandbox_size(width as usize, height as usize, self.no_braille);
                self.sandbox.resize(width, height);
            }
        }
    }

    fn handle_key_event(&mut self, e: KeyEvent) {
        match e.code {
            KeyCode::Char('c') if e.modifiers == KeyModifiers::CONTROL => self.quit(),
            KeyCode::Char(' ') => self.pause = !self.pause,
            KeyCode::Char(c) => {
                if let Some(pixel) = Pixel::iter().find(|pixel| pixel.hotkey() == c) {
                    self.active_pixel = pixel;
                }
            }
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, e: MouseEvent) {
        match e.kind {
            MouseEventKind::Down(_) => {
                self.mouse_down_event = Some(e);
            }
            MouseEventKind::Drag(_) => {
                self.mouse_down_event = Some(e);
                self.handle_mouse_down_event();
            }
            MouseEventKind::Up(_) => {
                self.mouse_down_event = None;
            }
            _ => {}
        }
    }

    fn handle_mouse_down_event(&mut self) {
        let Some(e) = self.mouse_down_event.as_ref() else {
            return;
        };
        if e.row == 0 || e.column == 0 {
            return;
        }
        // need to offset by the border
        let x = e.column as usize - 1;
        let y = e.row as usize - 1;

        match self.no_braille {
            false => {
                let x = x * 2;
                let y = y * 4;

                for i in 0..2 {
                    for j in 0..4 {
                        self.place_pixel(x + i, y + j);
                    }
                }
            }
            true => self.place_pixel(x, y),
        }
    }

    fn place_pixel(&mut self, x: usize, y: usize) {
        if x > self.sandbox.width - 1 || y > self.sandbox.height - 1 {
            return;
        }

        match self.active_pixel {
            Pixel::Void(_) => self.sandbox.place_pixel_force(self.active_pixel, x, y),
            _ => self.sandbox.place_pixel(self.active_pixel, x, y),
        }
    }
}

pub trait PixelHotkey {
    fn hotkey(&self) -> char;
}

impl PixelHotkey for Pixel {
    fn hotkey(&self) -> char {
        match self {
            Pixel::Sand(_) => '1',
            Pixel::Water(_) => '2',
            Pixel::Rock(_) => '3',
            Pixel::Steam(_) => '4',
            Pixel::Void(_) => '0',
        }
    }
}
