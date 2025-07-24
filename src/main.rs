use eframe::egui;
use cpal::traits::{HostTrait, DeviceTrait};

struct SoundboardApp {
    input_devices: Vec<String>,
    output_devices: Vec<String>,
    error: Option<String>,
}

impl SoundboardApp {
    fn new() -> Self {
        let host = cpal::default_host();
        let mut input_devices = Vec::new();
        let mut output_devices = Vec::new();
        let mut error = None;

        match host.input_devices() {
            Ok(devices) => {
                for device in devices {
                    input_devices.push(device.name().unwrap_or_else(|_| "Unknown".to_string()));
                }
            }
            Err(e) => error = Some(format!("Failed to enumerate input devices: {}", e)),
        }
        match host.output_devices() {
            Ok(devices) => {
                for device in devices {
                    output_devices.push(device.name().unwrap_or_else(|_| "Unknown".to_string()));
                }
            }
            Err(e) => error = Some(format!("Failed to enumerate output devices: {}", e)),
        }

        Self { input_devices, output_devices, error }
    }
}

impl eframe::App for SoundboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Boomcrab Soundboard");
            if let Some(ref err) = self.error {
                ui.colored_label(egui::Color32::RED, err);
            }
            ui.separator();
            ui.heading("Microphones (Input Devices)");
            if self.input_devices.is_empty() {
                ui.label("No input devices found.");
            } else {
                for dev in &self.input_devices {
                    ui.label(dev);
                }
            }
            ui.separator();
            ui.heading("Speakers (Output Devices)");
            if self.output_devices.is_empty() {
                ui.label("No output devices found.");
            } else {
                for dev in &self.output_devices {
                    ui.label(dev);
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Boomcrab Soundboard",
        options,
        Box::new(|_cc| Ok(Box::new(SoundboardApp::new()))),
    )
}
