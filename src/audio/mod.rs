pub mod pipewire;
use std::path::PathBuf;

#[derive(Debug)]
pub enum AudioError {
    DeviceNotFound(String),
    InitializationFailed(String),
    SoundNotFound(String),
    PlaybackError(String),
    NotSupported(String),
}

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub name: String,
    pub id: String,
    pub device_type: DeviceType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceType {
    Input,
    Output,
    Virtual,
}

#[derive(Debug)]
pub struct Sound {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
}

/// Trait that abstracts audio backend behavior across different operating systems.
pub trait AudioBackend {
    fn list_audio_outputs(&self) -> Result<Vec<AudioDevice>, AudioError>;
    fn list_audio_inputs(&self) -> Result<Vec<AudioDevice>, AudioError>;

    // fn set_input_device(&mut self, device_id: &str) -> Result<(), AudioError>;
    // fn set_output_device(&mut self, device_id: &str) -> Result<(), AudioError>;

    // fn create_virtual_mic(&mut self, name: &str) -> Result<AudioDevice, AudioError>;
    // fn destroy_virtual_mic(&mut self) -> Result<(), AudioError>;

    // fn load_sound(&mut self, path: PathBuf, name: String) -> Result<String, AudioError>;
    // fn unload_sound(&mut self, sound_id: &str) -> Result<(), AudioError>;
    // fn list_sounds(&self) -> Vec<Sound>;

    // fn play_sound(&mut self, sound_id: &str) -> Result<(), AudioError>;
    // fn pause_sound(&mut self, sound_id: &str) -> Result<(), AudioError>;
    // fn stop_sound(&mut self, sound_id: &str) -> Result<(), AudioError>;
    // fn stop_all(&mut self) -> Result<(), AudioError>;

    // fn set_sound_volume(&mut self, sound_id: &str, volume: f32) -> Result<(), AudioError>;
    // fn set_master_volume(&mut self, volume: f32) -> Result<(), AudioError>;

    // fn enable_mic_passthrough(&mut self, enabled: bool) -> Result<(), AudioError>;
    // fn set_mic_volume(&mut self, volume: f32) -> Result<(), AudioError>;
}

pub struct BoomCrabAudioInterface {
    backend: Box<dyn AudioBackend>,
}

impl BoomCrabAudioInterface {
    pub fn new() -> Result<Self, AudioError> {
        #[cfg(target_os = "linux")]
        let backend = Box::new(pipewire::PipeWireBackend::new()?) as Box<dyn AudioBackend>;

        // #[cfg(target_os = "windows")]
        // let backend = Box::new(wasapi::WasapiBackend::new()?) as Box<dyn AudioBackend>;

        // #[cfg(target_os = "macos")]
        // let backend = Box::new(coreaudio::CoreAudioBackend::new()?) as Box<dyn AudioBackend>;

        Ok(Self { backend })
    }

    pub fn list_audio_outputs(&self) -> Result<Vec<AudioDevice>, AudioError> {
        self.backend.list_audio_outputs()
    }

    pub fn list_audio_inputs(&self) -> Result<Vec<AudioDevice>, AudioError> {
        self.backend.list_audio_inputs()
    }
}

//     pub fn create_virtual_mic(&mut self, name: &str) -> Result<AudioDevice, AudioError> {
//         self.backend.create_virtual_mic(name)
//     }

//     pub fn play_sound(&mut self, sound_id: &str) -> Result<(), AudioError> {
//         self.backend.play_sound(sound_id)
//     }

//     fn list_audio_outputs(&self) -> Result<Vec<AudioDevice>, AudioError>;
//     fn list_audio_inputs(&self) -> Result<Vec<AudioDevice>, AudioError>;

//     fn set_input_device(&mut self, device_id: &str) -> Result<(), AudioError>;
//     fn set_output_device(&mut self, device_id: &str) -> Result<(), AudioError>;

//     fn create_virtual_mic(&mut self, name: &str) -> Result<AudioDevice, AudioError>;
//     fn destroy_virtual_mic(&mut self) -> Result<(), AudioError>;

//     fn load_sound(&mut self, path: PathBuf, name: String) -> Result<String, AudioError>;
//     fn unload_sound(&mut self, sound_id: &str) -> Result<(), AudioError>;
//     fn list_sounds(&self) -> Vec<Sound>;

//     fn play_sound(&mut self, sound_id: &str) -> Result<(), AudioError>;
//     fn pause_sound(&mut self, sound_id: &str) -> Result<(), AudioError>;
//     fn stop_sound(&mut self, sound_id: &str) -> Result<(), AudioError>;
//     fn stop_all(&mut self) -> Result<(), AudioError>;

//     fn set_sound_volume(&mut self, sound_id: &str, volume: f32) -> Result<(), AudioError>;
//     fn set_master_volume(&mut self, volume: f32) -> Result<(), AudioError>;

//     fn enable_mic_passthrough(&mut self, enabled: bool) -> Result<(), AudioError>;
//     fn set_mic_volume(&mut self, volume: f32) -> Result<(), AudioError>;
// }
