//! This feature allows opus encoding and decoding for efficient byte transfer, while not sacirifising audio quality.

use encode::EncoderType;
pub mod decode;
pub mod encode;

/// Re-export the opus crate.
pub use opus;

/// The encoded sound packet.
/// Contains useful information about the encoded packet.
#[derive(Debug)]
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
    /// The count of bytes of the packet.
    pub samples_per_frame: u64,
}
