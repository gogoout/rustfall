use crate::display::event::Event;
use crate::engine::pixel::rock::Rock;
use crate::engine::pixel::sand::Sand;
use crate::engine::pixel::steam::Steam;
use crate::engine::pixel::void::Void;
use crate::engine::pixel::water::Water;
use crate::engine::pixel::Pixel;
use crate::engine::sandbox::Sandbox;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind};

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
        Self {
            should_quit: false,
            sandbox: Sandbox::new(width, height),
            active_pixel: Default::default(),
            no_braille,
            mouse_down_event: None,
            pause: false,
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
            _ => {}
        }
    }

    fn handle_key_event(&mut self, e: KeyEvent) {
        match e.code {
            KeyCode::Char('c') if e.modifiers == KeyModifiers::CONTROL => self.quit(),
            KeyCode::Char(' ') => self.pause = !self.pause,
            KeyCode::Char(c) => match c {
                '1' => self.active_pixel = Sand.into(),
                '2' => self.active_pixel = Rock.into(),
                '3' => self.active_pixel = Water.into(),
                '4' => self.active_pixel = Steam.into(),
                '0' => self.active_pixel = Void.into(),
                _ => {}
            },
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, e: MouseEvent) {
        match e.kind {
            MouseEventKind::Down(_) | MouseEventKind::Drag(_) => {
                self.mouse_down_event = Some(e);
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
        match self.no_braille {
            false => {
                let x = e.column as usize * 2;
                let y = e.row as usize * 4;
                for i in 0..2 {
                    for j in 0..4 {
                        self.place_pixel(x + i, y + j);
                    }
                }
            }
            true => self.place_pixel(e.column as usize, e.row as usize),
        }
    }

    fn place_pixel(&mut self, x: usize, y: usize) {
        match self.active_pixel {
            Pixel::Void(_) => self
                .sandbox
                .place_pixel_force(self.active_pixel.clone(), x, y),
            _ => self.sandbox.place_pixel(self.active_pixel.clone(), x, y),
        }
    }
}
