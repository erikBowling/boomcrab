use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use super::App;

pub struct StartScreen;

impl StartScreen {
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
        let title = Paragraph::new("🦀 BoomCrab 🦀")
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

        let welcome_text = vec![
            Line::from(""),
            Line::from(Span::styled(
                "Welcome to BoomCrab!",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("A powerful audio management tool for Linux."),
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                "Quick Start:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from("  • Press [2] to view audio devices"),
            Line::from("  • Press [3] for configuration"),
            Line::from("  • Press [q] or [Esc] to quit"),
        ];

        let welcome = Paragraph::new(welcome_text)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Start Screen")
                    .border_style(Style::default().fg(Color::White)),
            );
        frame.render_widget(welcome, chunks[1]);

        Self::render_footer(frame, chunks[2]);
    }

    fn render_footer(frame: &mut Frame, area: Rect) {
        let footer = Paragraph::new("Press [1] Start | [2] Main | [3] Config | [q] Quit")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, area);
    }
}
