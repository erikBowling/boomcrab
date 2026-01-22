use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, Paragraph},
};

use super::app::App;
use super::components::footer::render_footer;
use crate::audio::DeviceType;

pub struct HomePage;

impl HomePage {
    pub fn render(frame: &mut Frame, app: &App) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new("Home")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::White)),
            );

        frame.render_widget(title, chunks[0]);
        frame.render_widget(Block::new(), chunks[1]);
        render_footer(frame, chunks[2]);
    }
}
