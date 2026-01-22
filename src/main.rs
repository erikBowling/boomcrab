mod audio;
mod ui;

use audio::BoomCrabAudioInterface;
use ui::{UiAction, app::App, restore_terminal, setup_terminal};

use audio::AudioError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audio_interface = BoomCrabAudioInterface::new().unwrap();

    let mut terminal = setup_terminal()?;
    let mut ui_app = App::new();
    ui_app.audio_outputs = audio_interface.list_audio_outputs().unwrap_or_default();
    ui_app.audio_inputs = audio_interface.list_audio_inputs().unwrap_or_default();

    loop {
        terminal.draw(|frame| ui_app.render(frame))?;

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

    // Restore terminal
    restore_terminal()?;

    Ok(())
}
