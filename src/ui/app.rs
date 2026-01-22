use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use super::{Page, UiAction, config::ConfigPage, home::HomePage};
use crate::audio::AudioDevice;

pub struct App {
    pub current_page: Page,
    pub audio_outputs: Vec<AudioDevice>,
    pub audio_inputs: Vec<AudioDevice>,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_page: Page::Home,
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
                self.current_page = Page::Home;
                UiAction::None
            }
            KeyCode::Char('2') => {
                self.current_page = Page::Config;
                UiAction::None
            }
            KeyCode::Char('r') => UiAction::RefreshAudioDevices,
            _ => UiAction::None,
        }
    }

    pub fn render(&self, frame: &mut ratatui::Frame) {
        match self.current_page {
            Page::Home => HomePage::render(frame, self),
            Page::Config => ConfigPage::render(frame, self),
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
