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
    active_pixel: Pixel,
}

impl State {
    /// Constructs a new instance of [`State`].
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            should_quit: false,
            sandbox: Sandbox::new(width, height),
            active_pixel: Default::default(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn update(&mut self, event: Event) {
        match event {
            Event::Tick => self.tick(),
            Event::Key(key) => self.handle_key_event(key),
            Event::Mouse(mouse) => self.handle_mouse_event(mouse),
            _ => {}
        }
    }

    fn handle_key_event(&mut self, e: KeyEvent) {
        match e.code {
            KeyCode::Char('c') if e.modifiers == KeyModifiers::CONTROL => self.quit(),
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
            MouseEventKind::Down(_) | MouseEventKind::Drag(_) => match self.active_pixel {
                Pixel::Void(_) => {
                    self.sandbox
                        .place_pixel_force(Void.into(), e.column as usize, e.row as usize)
                }
                _ => self
                    .sandbox
                    .place_pixel(Void.into(), e.column as usize, e.row as usize),
            },
            _ => {}
        }
    }
}
