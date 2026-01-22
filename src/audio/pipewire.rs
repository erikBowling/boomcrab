use pipewire as pw;
use pw::context::ContextRc;
use pw::main_loop::MainLoopRc;
use std::sync::{Arc, Mutex};
use std::thread;

use super::{AudioBackend, AudioDevice, AudioError, DeviceType};

/// PipeWire backend implementation for Linux audio
pub struct PipeWireBackend {}

impl PipeWireBackend {
    /// Creates a new PipeWire backend instance
    pub fn new() -> Result<Self, AudioError> {
        // Initialize PipeWire library
        pw::init();

        Ok(PipeWireBackend {})
    }

    /// Internal helper to list devices by media class
    fn list_devices_by_class(&self, media_class: &str) -> Result<Vec<AudioDevice>, AudioError> {
        let devices = Arc::new(Mutex::new(Vec::new()));
        let devices_clone = Arc::clone(&devices);

        let main_loop = MainLoopRc::new(None).map_err(|e| {
            AudioError::InitializationFailed(format!("Failed to create main loop: {}", e))
        })?;

        let context = ContextRc::new(&main_loop, None).map_err(|e| {
            AudioError::InitializationFailed(format!("Failed to create context: {}", e))
        })?;

        let core = context.connect_rc(None).map_err(|e| {
            AudioError::InitializationFailed(format!("Failed to connect to PipeWire: {}", e))
        })?;

        let registry = core.get_registry_rc().map_err(|e| {
            AudioError::InitializationFailed(format!("Failed to get registry: {}", e))
        })?;

        let target_media_class = media_class.to_string();

        // Registry listener to capture devices
        let _listener = registry
            .add_listener_local()
            .global(move |global| {
                if let Some(props) = &global.props {
                    // Look for devices matching the specified media class
                    if let Some(media_class) = props.get("media.class") {
                        if media_class == target_media_class {
                            let id = global.id.to_string();
                            let name = props
                                .get("node.name")
                                .or_else(|| props.get("object.serial"))
                                .unwrap_or("Unknown")
                                .to_string();

                            // Determine device type based on media class
                            let device_type = if target_media_class == "Audio/Sink" {
                                DeviceType::Output
                            } else if target_media_class == "Audio/Source" {
                                DeviceType::Input
                            } else if target_media_class.contains("Virtual") {
                                DeviceType::Virtual
                            } else {
                                DeviceType::Input // Default
                            };

                            let device = AudioDevice {
                                name,
                                id,
                                device_type,
                            };

                            devices_clone.lock().unwrap().push(device);
                        }
                    }
                }
            })
            .register();

        // Run the loop briefly to collect devices
        let main_loop_clone = main_loop.clone();
        let timer = main_loop.loop_().add_timer(move |_| {
            main_loop_clone.quit();
        });

        // Wait 500ms to gather all devices
        timer.update_timer(Some(std::time::Duration::from_millis(500)), None);

        main_loop.run();

        let result = devices.lock().unwrap().clone();
        Ok(result)
    }
}

impl AudioBackend for PipeWireBackend {
    /// Lists all available audio output devices (sinks)
    fn list_audio_outputs(&self) -> Result<Vec<AudioDevice>, AudioError> {
        self.list_devices_by_class("Audio/Sink")
    }

    /// Lists all available audio input devices (sources)
    fn list_audio_inputs(&self) -> Result<Vec<AudioDevice>, AudioError> {
        self.list_devices_by_class("Audio/Source")
    }
}

// Keep the VirtualMicrophone struct for future use
pub struct VirtualMicrophone {
    main_loop_thread: Option<thread::JoinHandle<()>>,
    terminate_sender: Option<pw::channel::Sender<()>>,
}

impl VirtualMicrophone {
    /// Creates a new virtual microphone (input stream)
    ///
    /// # Arguments
    /// * `name` - Name for the virtual microphone
    /// * `description` - Description for the virtual microphone
    /// * `channels` - Number of audio channels (e.g., 2 for stereo)
    pub fn new(
        name: &str,
        description: &str,
        channels: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        pw::init();

        let (terminate_sender, terminate_receiver) = pw::channel::channel::<()>();

        let name = name.to_string();
        let description = description.to_string();

        // TODO: Probably move thread out
        let main_loop_thread = thread::spawn(move || {
            let main_loop = MainLoopRc::new(None).expect("Failed to initialize PipeWire main loop");

            let context =
                ContextRc::new(&main_loop, None).expect("Failed to create PipeWire context");

            let core = context
                .connect(None)
                .expect("Failed to connect to PipeWire");

            // Set up channel positions based on number of channels
            let audio_position = if channels == 1 {
                "[ MONO ]"
            } else if channels == 2 {
                "[ FL FR ]"
            } else {
                // For more channels, you'd need to specify more positions
                "[ FL FR ]"
            };

            // Create a virtual audio source (microphone)
            let props = pw::properties::properties! {
                "factory.name" => "support.null-audio-sink",
                "node.name" => name.as_str(),
                "node.description" => description.as_str(),
                "media.class" => "Audio/Source/Virtual",
                "audio.position" => audio_position,
                "audio.channels" => channels.to_string().as_str(),
                "object.linger" => "false", // Destroy the node on drop
            };

            let _node = core
                .create_object::<pw::node::Node>("adapter", &props)
                .expect("Failed to create virtual microphone");

            println!("Virtual microphone '{}' created", description);

            // Set up termination handler
            let _receiver = terminate_receiver.attach(main_loop.loop_(), {
                let main_loop = main_loop.clone();
                move |_| main_loop.quit()
            });

            // Run the main loop (blocks until terminate signal)
            main_loop.run();

            println!("Virtual microphone '{}' terminated", description);
        });

        // Give the thread a moment to initialize
        std::thread::sleep(std::time::Duration::from_millis(100));

        Ok(VirtualMicrophone {
            main_loop_thread: Some(main_loop_thread),
            terminate_sender: Some(terminate_sender),
        })
    }

    /// Disconnect and destroy the virtual microphone
    pub fn disconnect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(sender) = self.terminate_sender.take() {
            sender
                .send(())
                .map_err(|_| "Failed to send termination signal")?;
        }

        if let Some(thread) = self.main_loop_thread.take() {
            thread.join().map_err(|_| "Thread join failed")?;
        }

        Ok(())
    }
}

impl Drop for VirtualMicrophone {
    fn drop(&mut self) {
        // Best effort termination - ignore errors in drop
        if let Some(sender) = self.terminate_sender.take() {
            sender.send(()).ok();
        }
    }
}
