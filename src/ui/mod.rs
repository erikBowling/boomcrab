use eframe::egui;
use crate::settings::BoomCrabSettings;

pub struct SoundboardApp {
    settings: BoomCrabSettings,
    error: Option<String>,
}

impl SoundboardApp {
    pub fn new() -> Self {
        Self {
            settings: BoomCrabSettings::new().unwrap_or_else(|_| {
                eprintln!("Failed to load settings, using defaults");
                BoomCrabSettings {
                    sound_files_directory: String::new()
                }
            }),
            error: None,
        }
    }
}

impl eframe::App for SoundboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Boomcrab Soundboard");
            
            if let Some(error) = &self.error {
                ui.colored_label(egui::Color32::RED, error);
            }
            
            ui.label("Sound files directory:");
            ui.text_edit_singleline(&mut self.settings.sound_files_directory);
            
            if ui.button("Save Settings").clicked() {
                if let Err(e) = self.settings.save_to_file() {
                    self.error = Some(format!("Failed to save settings: {}", e));
                } else {
                    self.error = None;
                }
            }
        });
    }
}