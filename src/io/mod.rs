//! Traits, functions and type definitions for functioning audio I/O.
//! The I/O feature provides types and functions for recording and playbacking audio aswell as handling the data.

use ::cpal::{
    traits::{DeviceTrait, HostTrait},
    DefaultStreamConfigError, Device, Host, SupportedStreamConfig,
};

pub mod playback;
pub mod record;

/// Wrapper type for differentiating [`OutputDevice`] from [`InputDevice`] granted the user passes them in right when creating an [`AudioDevice`].
pub type OutputDevice = Device;

/// Wrapper type for differentiating [`OutputDevice`] from [`InputDevice`] granted the user passes them in right when creating an [`AudioDevice`].
pub type InputDevice = Device;

//Re-export the cpal crate.
pub use cpal;

/// The host's audio input and output devices.
#[allow(missing_debug_implementations)]
pub struct HostDevice {
    /// The output device of the host.
    pub output: Option<OutputDevice>,
    /// The input device of the host.
    pub input: Option<InputDevice>,
}

impl HostDevice {
    /// Creates a new [`AudioDevice`] instance.
    pub fn new(output: Option<Device>, input: Option<Device>) -> Self {
        Self { output, input }
    }

    /// Gets the `input` device's default configuration
    pub fn get_input_config(
        &self,
    ) -> Option<Result<SupportedStreamConfig, DefaultStreamConfigError>> {
        self.input
            .clone()
            .map(|input_device| input_device.default_input_config())
    }

    /// Gets the `output` device's default configuration
    pub fn get_output_config(
        &self,
    ) -> Option<Result<SupportedStreamConfig, DefaultStreamConfigError>> {
        self.output
            .clone()
            .map(|input_device| input_device.default_output_config())
    }
}

pub use cpal::{available_hosts, default_host, host_from_id};

/// Get the default audio devices of the host. If one doesn't exist it will get initialized with `None`.
pub fn get_audio_device(device: Host) -> HostDevice {
    HostDevice {
        input: device.default_input_device(),
        output: device.default_output_device(),
    }
}

/// Shows the encoder type of the packet.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, deepsize::DeepSizeOf)]
pub enum EncoderType {
    /// The encoder of this packet was [`opus`].
    /// The inner value contains whether.
    Opus(bool),
}

/// The encoded sound packet.
/// Contains useful information about the encoded packet.
#[derive(Debug, deepsize::DeepSizeOf)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SoundPacket {
    /// The Encoder's type which this [`SoundPacket`] got encoded with.
    pub encoder_type: EncoderType,
    /// The sample rate of the encoded packet.
    pub sample_rate: u32,
    /// The channel count of the encoded packet.
    pub channels: u32,
    /// The bytes of the encoded sound packet.
    pub bytes: Vec<u8>,
    /// The count of samples per frame.
    pub samples_per_frame: u64,
}

