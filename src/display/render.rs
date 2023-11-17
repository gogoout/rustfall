use itertools::Itertools;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::Marker;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::canvas::{Canvas, Painter, Shape};
use ratatui::widgets::{BorderType, List, ListItem, ListState};
use ratatui::{
    prelude::Frame,
    style::Color,
    symbols,
    widgets::{Block, Borders},
};
use std::sync::OnceLock;
use strum::IntoEnumIterator;

use crate::display::state::{PixelHotkey, State};
use crate::engine::pixel::{BasicPixel, Pixel};
use crate::engine::sandbox::Sandbox;

pub struct Renderer {
    no_braille: bool,
}
impl Renderer {
    pub fn new(no_braille: bool) -> Self {
        Self { no_braille }
    }

    fn pixel_bar_width() -> u16 {
        20
    }
    fn list_items() -> &'static [ListItem<'static>] {
        static CELL: OnceLock<Vec<ListItem<'static>>> = OnceLock::new();
        CELL.get_or_init(|| {
            Pixel::iter()
                .sorted_by_key(|pixel| pixel.hotkey())
                .map(|pixel| {
                    ListItem::new(format!(
                        "[{}]{}",
                        pixel.hotkey(),
                        PixelDisplay::name(&pixel)
                    ))
                })
                .collect::<Vec<_>>()
        })
    }

    pub fn render(&self, state: &State, f: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Min(10),
                Constraint::Max(Self::pixel_bar_width()),
            ])
            .split(f.size());

        f.render_widget(
            Canvas::default()
                .block(
                    Block::default()
                        .border_set(symbols::border::PLAIN)
                        .borders(Borders::ALL)
                        .title("Rustfull"),
                )
                .marker(match self.no_braille {
                    false => Marker::Braille,
                    true => Marker::Dot,
                })
                .paint(|ctx| {
                    ctx.draw(&state.sandbox);
                }),
            layout[0],
        );

        let list_items = Self::list_items();
        let mut list_state = ListState::default().with_selected(
            Pixel::iter()
                .sorted_by_key(|pixel| pixel.hotkey())
                .position(|p| p == state.active_pixel),
        );

        f.render_stateful_widget(
            List::new(list_items)
                .block(
                    Block::default()
                        .border_set(symbols::border::PLAIN)
                        .borders(Borders::TOP | Borders::RIGHT | Borders::BOTTOM)
                        .title("Pixels"),
                )
                .style(Style::default().fg(Color::White))
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::ITALIC)
                        .bg(Color::DarkGray),
                )
                .highlight_symbol(">>"),
            layout[1],
            &mut list_state,
        );
    }

    pub fn sandbox_size(&self, rect: Rect) -> (usize, usize) {
        let width = (rect.width - Self::pixel_bar_width()) as usize;
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
    fn name(&self) -> &'static str;
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

    fn name(&self) -> &'static str {
        BasicPixel::name(self)
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
