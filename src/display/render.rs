use crate::display::state::State;
use ratatui::widgets::Dataset;
use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render(state: &mut State, f: &mut Frame) {
    f.render_widget(
        Dataset::default().style(Style::default().fg(Color::White)),
        f.size(),
    );
}
