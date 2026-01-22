use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

pub fn render_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new("Press [1] Home | [2] Config | [r] Refresh | [q][esc] Quit")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, area);
}
