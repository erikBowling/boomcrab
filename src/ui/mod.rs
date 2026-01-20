mod config;
mod main_screen;
mod start;

use ratatui::{
    Terminal,
    backend::Backend,
    crossterm::{
        ExecutableCommand,
        event::{self, Event, KeyCode, KeyEventKind},
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
};
use std::io::{self, stdout};

use crate::audio::AudioDevice;

pub use config::ConfigScreen;
pub use main_screen::MainScreen;
pub use start::StartScreen;

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Start,
    Main,
    Config,
}

/// Actions that the UI wants the application to perform
#[derive(Debug, Clone, PartialEq)]
pub enum UiAction {
    None,
    RefreshAudioDevices,
    Quit,
}

pub struct App {
    pub current_page: Page,
    pub audio_outputs: Vec<AudioDevice>,
    pub audio_inputs: Vec<AudioDevice>,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_page: Page::Start,
            audio_outputs: Vec::new(),
            audio_inputs: Vec::new(),
        }
    }

    /// Update the audio device lists with new data
    pub fn update_audio_devices(&mut self, outputs: Vec<AudioDevice>, inputs: Vec<AudioDevice>) {
        self.audio_outputs = outputs;
        self.audio_inputs = inputs;
    }

    /// Handle keyboard input and return an action for the main app to handle
    pub fn handle_key_event(&mut self, key: KeyCode) -> UiAction {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => UiAction::Quit,
            KeyCode::Char('1') => {
                self.current_page = Page::Start;
                UiAction::None
            }
            KeyCode::Char('2') => {
                self.current_page = Page::Main;
                UiAction::RefreshAudioDevices
            }
            KeyCode::Char('3') => {
                self.current_page = Page::Config;
                UiAction::None
            }
            KeyCode::Char('r') if self.current_page == Page::Main => UiAction::RefreshAudioDevices,
            _ => UiAction::None,
        }
    }

    pub fn render(&self, frame: &mut ratatui::Frame) {
        match self.current_page {
            Page::Start => StartScreen::render(frame, self),
            Page::Main => MainScreen::render(frame, self),
            Page::Config => ConfigScreen::render(frame, self),
        }
    }

    pub fn poll_events(&mut self) -> io::Result<UiAction> {
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    return Ok(self.handle_key_event(key.code));
                }
            }
        }
        Ok(UiAction::None)
    }
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
