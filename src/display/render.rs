use std::sync::OnceLock;

use itertools::Itertools;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::prelude::Marker;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::block::Title;
use ratatui::widgets::canvas::{Canvas, Painter, Shape};
use ratatui::widgets::{List, ListItem, ListState};
use ratatui::{
    prelude::Frame,
    style::Color,
    symbols,
    widgets::{Block, Borders},
};
use strum::IntoEnumIterator;

use crate::display::state::{PixelHotkey, State};
use crate::engine::pixel::{Pixel, PixelFundamental};
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
                .map(|pixel| ListItem::new(format!("[{}]{}", pixel.hotkey(), pixel.name())))
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
                        .title("Rustfall")
                        .title(
                            Title::from(format!(
                                "({} * {})",
                                state.sandbox.width, state.sandbox.height
                            ))
                            .alignment(Alignment::Center),
                        )
                        .title(
                            Title::from(format!("{:.2} fps", state.sandbox.fps()))
                                .alignment(Alignment::Right),
                        ),
                )
                .marker(match self.no_braille {
                    false => Marker::Braille,
                    true => Marker::Block,
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
                .highlight_symbol("[x]"),
            layout[1],
            &mut list_state,
        );
    }

    pub fn sandbox_size(width: usize, height: usize) -> (usize, usize) {
        let width = width - Self::pixel_bar_width() as usize;
        let height = height;
        let canvas_width = width - 2;
        let canvas_height = height - 2;

        (canvas_width, canvas_height)
    }
}

pub trait PixelDisplay {
    fn display(&self) -> Color;
}

impl PixelDisplay for Pixel {
    fn display(&self) -> Color {
        match self {
            // light blue
            Pixel::Steam(_) => Color::Indexed(69),
            // darker yellow
            Pixel::Sand(_) => Color::Indexed(214),
            // grey
            Pixel::Rock(_) => Color::Indexed(254),
            Pixel::Water(_) => Color::Blue,
            Pixel::Void(_) => Color::Black,
            Pixel::Fire(_) => Color::Red,
            Pixel::Wood(val) => {
                if val.is_burning() {
                    Color::Indexed(202)
                } else {
                    Color::Yellow
                }
            }
            Pixel::Ice(_) => Color::Indexed(195),
        }
    }
}

impl Shape for Sandbox {
    fn draw(&self, painter: &mut Painter) {
        for (idx, pixel) in self.pixels.iter().enumerate() {
            if let Pixel::Void(_) = pixel.pixel() {
                continue;
            }
            let (x, y) = self.index_to_coordinates(idx);
            painter.paint(x, y, pixel.pixel().display());
        }
    }
}
