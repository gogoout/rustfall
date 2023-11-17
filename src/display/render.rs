use ratatui::layout::Rect;
use ratatui::prelude::Marker;
use ratatui::widgets::canvas::{Canvas, Painter, Shape};
use ratatui::widgets::Widget;
use ratatui::{
    prelude::Frame,
    style::Color,
    widgets::{Block, Borders},
};

use crate::display::state::State;
use crate::engine::pixel::Pixel;
use crate::engine::sandbox::Sandbox;

pub struct Renderer {
    no_braille: bool,
}
impl Renderer {
    pub fn new(no_braille: bool) -> Self {
        Self { no_braille }
    }

    pub fn render(&self, state: &State, f: &mut Frame) {
        f.render_widget(
            Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Rustfull"))
                .marker(match self.no_braille {
                    false => Marker::Braille,
                    true => Marker::Dot,
                })
                .paint(|ctx| {
                    ctx.draw(&state.sandbox);
                }),
            f.size(),
        );
    }

    pub fn sandbox_size(&self, rect: Rect) -> (usize, usize) {
        let width = rect.width as usize;
        let height = rect.height as usize;
        let canvas_width = width - 2;
        let canvas_height = height - 2;

        match self.no_braille {
            false => (canvas_width * 2, canvas_height * 4),
            true => (canvas_width, canvas_height),
        }
    }
}

pub trait PixelDisplay {
    fn display(&self) -> Color;
}

impl PixelDisplay for Pixel {
    fn display(&self) -> Color {
        match self {
            Pixel::Steam(_) => Color::LightBlue,
            // darker yellow
            Pixel::Sand(_) => Color::LightYellow,
            // grey
            Pixel::Rock(_) => Color::Indexed(254),
            Pixel::Water(_) => Color::Blue,
            Pixel::Void(_) => Color::Black,
        }
    }
}

impl Shape for Sandbox {
    fn draw(&self, painter: &mut Painter) {
        for (idx, pixel) in self.pixels.iter().enumerate() {
            if let Pixel::Void(_) = pixel {
                continue;
            }
            let (x, y) = self.index_to_coordinates(idx);
            painter.paint(x, y, pixel.display());
        }
    }
}
