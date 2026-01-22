use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use super::app::App;
use super::components::footer::render_footer;

pub struct ConfigPage;

impl ConfigPage {
    pub fn render(frame: &mut Frame, _app: &App) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(area);

        // Title
        let title = Paragraph::new("Configuration")
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

        let config_text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Settings",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("Configuration options will be available here."),
            Line::from(""),
            Line::from(Span::styled(
                "Coming soon!",
                Style::default().fg(Color::Green),
            )),
        ];

        let config = Paragraph::new(config_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Config Screen")
                    .border_style(Style::default().fg(Color::Magenta)),
            );
        frame.render_widget(config, chunks[1]);
        render_footer(frame, chunks[2]);
    }
}
