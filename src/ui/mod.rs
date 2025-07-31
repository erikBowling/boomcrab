use eframe::egui;
use crate::settings::BoomCrabSettings;
use std::path::Path;
use std::fs;

pub struct SoundboardApp {
    settings: BoomCrabSettings,
    error: Option<String>,
}

impl SoundboardApp {
    pub fn new() -> Self {
        Self {
            settings: BoomCrabSettings::new().expect("Failed to load settings"),
            error: None,
        }
    }

    fn get_sound_files(&self) -> Vec<String> {
        let sound_dir = &self.settings.sound_files_directory;
        if sound_dir.is_empty() {
            return Vec::new();
        }

        let path = Path::new(sound_dir);
        if !path.exists() || !path.is_dir() {
            return Vec::new();
        }

        let mut sound_files = Vec::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_path = entry.path();
                    if file_path.is_file() {
                        // Check for common audio file extensions
                        if let Some(extension) = file_path.extension() {
                            let ext = extension.to_string_lossy().to_lowercase();
                            if matches!(ext.as_str(), "mp3" | "wav" | "ogg" | "flac" | "m4a" | "aac") {
                                if let Some(file_name) = file_path.file_name() {
                                    sound_files.push(file_name.to_string_lossy().to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        sound_files
    }

    fn render_sound_squares(&self, ui: &mut egui::Ui) {
        let sound_files = self.get_sound_files();
        
        if sound_files.is_empty() {
            ui.label("No sound files found in the specified directory.");
            return;
        }

        ui.heading("Sound Files");
        
        // Calculate grid layout
        let available_width = ui.available_width();
        let square_size = 120.0;
        let spacing = 10.0;
        let squares_per_row = ((available_width - spacing) / (square_size + spacing)).max(1.0) as usize;
        
        egui::Grid::new("sound_squares")
            .spacing([spacing, spacing])
            .show(ui, |ui| {
                for (index, file_name) in sound_files.iter().enumerate() {
                    if index > 0 && index % squares_per_row == 0 {
                        ui.end_row();
                    }
                    
                    // Create a square button for each sound file
                    let response = ui.add_sized(
                        [square_size, square_size],
                        egui::Button::new(egui::RichText::new(file_name).size(12.0))
                    );
                    
                    // Handle click events
                    if response.clicked() {
                        // TODO: Implement sound playback
                        println!("Clicked on sound file: {}", file_name);
                    }
                    
                    // Add hover effect
                    if response.hovered() {
                        ui.painter().rect_stroke(
                            response.rect,
                            2.0,
                            egui::Stroke::new(2.0, egui::Color32::WHITE),
                            egui::StrokeKind::Outside
                        );
                    }
                }
            });
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
            
            ui.separator();
            
            // Render the sound squares
            self.render_sound_squares(ui);
        });
    }
}