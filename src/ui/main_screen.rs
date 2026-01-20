use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use super::App;
use crate::audio::DeviceType;

pub struct MainScreen;

impl MainScreen {
    pub fn render(frame: &mut Frame, app: &App) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new("Audio Devices")
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

        let device_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        Self::render_output_devices(frame, app, device_chunks[0]);

        Self::render_input_devices(frame, app, device_chunks[1]);

        Self::render_footer(frame, chunks[2]);
    }

    fn render_output_devices(frame: &mut Frame, app: &App, area: Rect) {
        let output_items: Vec<ListItem> = app
            .audio_outputs
            .iter()
            .map(|device| {
                let icon = match device.device_type {
                    DeviceType::Output => "🔊",
                    DeviceType::Virtual => "🎧",
                    _ => "🔉",
                };

                let line = Line::from(vec![
                    Span::raw(icon),
                    Span::raw(" "),
                    Span::styled(&device.name, Style::default().fg(Color::White)),
                ]);

                ListItem::new(line)
            })
            .collect();

        let output_count = app.audio_outputs.len();
        let title = format!("Output Devices ({})", output_count);

        let output_list = List::new(output_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::default().fg(Color::Green)),
            )
            .style(Style::default().fg(Color::White));

        frame.render_widget(output_list, area);
    }

    fn render_input_devices(frame: &mut Frame, app: &App, area: Rect) {
        let input_items: Vec<ListItem> = app
            .audio_inputs
            .iter()
            .map(|device| {
                let icon = match device.device_type {
                    DeviceType::Input => "🎤",
                    DeviceType::Virtual => "🎧",
                    _ => "🔉",
                };

                let line = Line::from(vec![
                    Span::raw(icon),
                    Span::raw(" "),
                    Span::styled(&device.name, Style::default().fg(Color::White)),
                ]);

                ListItem::new(line)
            })
            .collect();

        let input_count = app.audio_inputs.len();
        let title = format!("Input Devices ({})", input_count);

        let input_list = List::new(input_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .style(Style::default().fg(Color::White));

        frame.render_widget(input_list, area);
    }

    fn render_footer(frame: &mut Frame, area: Rect) {
        let footer =
            Paragraph::new("Press [1] Start | [2] Main | [3] Config | [q] Quit | [r] Refresh")
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::default().borders(Borders::ALL));
        frame.render_widget(footer, area);
    }
}
