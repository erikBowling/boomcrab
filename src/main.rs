mod settings;
mod ui;
mod audio;

use ui::SoundboardApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Boomcrab Soundboard",
        options,
        Box::new(|_cc| Ok(Box::new(SoundboardApp::new()))),
    )
}
