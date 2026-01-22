pub mod app;
mod config;
mod home;

mod components {
    pub mod footer;
}

use ratatui::{
    Terminal,
    crossterm::{
        ExecutableCommand,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::io::{self, stdout};

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Home,
    Config,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UiAction {
    None,
    RefreshAudioDevices,
    Quit,
}

pub fn setup_terminal() -> io::Result<Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>>
{
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let terminal = Terminal::new(ratatui::backend::CrosstermBackend::new(stdout()))?;
    Ok(terminal)
}

pub fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
