//! Traits, functions and type definitions for functioning audio I/O.
//! The I/O feature provides types and functions for recording and playbacking audio aswell as handling the data.

use cpal::{traits::HostTrait, Device, Host};

pub mod playback;
pub mod record;

/// Wrapper type for differentiating [`OutputDevice`] from [`InputDevice`] granted the user passes them in right when creating an [`AudioDevice`].
pub type OutputDevice = Device;

/// Wrapper type for differentiating [`OutputDevice`] from [`InputDevice`] granted the user passes them in right when creating an [`AudioDevice`].
pub type InputDevice = Device;

#[allow(missing_debug_implementations)]
/// The host's audio input and output devices.
pub struct AudioDevice {
    /// The output device of the host.
    pub output: Option<OutputDevice>,
    /// The input device of the host.
    pub input: Option<InputDevice>,
}

impl AudioDevice {
    /// Creates a new [`AudioDevice`] instance.
    pub fn new(output: Option<Device>, input: Option<Device>) -> Self {
        Self { output, input }
    }
}

pub use cpal::{available_hosts, default_host, host_from_id};

/// Get the default audio devices of the host. If one doesn't exist it will get initialized with `None`.
pub fn get_audio_device(device: Host) -> AudioDevice {
    AudioDevice {
        input: device.default_input_device(),
        output: device.default_output_device(),
    }
}
