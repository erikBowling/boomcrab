mod audio;
mod ui;

use audio::BoomCrabAudioInterface;
use ui::{App, UiAction, restore_terminal, setup_terminal};

use crate::audio::AudioError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_interface = BoomCrabAudioInterface::new().unwrap();

    let mut terminal = setup_terminal()?;

    let mut ui_app = App::new();

    let result = run_app(&mut terminal, &mut ui_app, &audio_interface);

    // Restore terminal
    restore_terminal()?;

    Ok(())
}

fn run_app(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    ui_app: &mut App,
    audio_interface: &BoomCrabAudioInterface,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| ui_app.render(frame))?;

        // Poll for events and get the action the UI wants us to perform
        match ui_app.poll_events()? {
            UiAction::Quit => break,
            UiAction::RefreshAudioDevices => {
                let outputs = audio_interface.list_audio_outputs().unwrap_or_default();
                let inputs = audio_interface.list_audio_inputs().unwrap_or_default();
                ui_app.update_audio_devices(outputs, inputs);
            }
            UiAction::None => {}
        }
    }

    Ok(())
}
