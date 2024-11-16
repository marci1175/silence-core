//! Eanbles raw sample encoding to opus

use cpal::SupportedStreamConfig;
use opus::Encoder;

use super::SoundPacket;

///
/// Create an [`opus`] encoder.
///
/// # Behavior
/// Creates an [`opus`] encoder from a [`SupportedStreamConfig`] to know the host's correct configurations, and an [`opus::Application`] to know which mode the user desires.
///
/// # Error
/// Returns an error if some kind of error occured while creating the [`Encoder`].
/// Example: invalid configurations were found (highly unlikely).
///
pub fn create_opus_encoder(
    config: SupportedStreamConfig,
    opus_mode: opus::Application,
    bitrate: opus::Bitrate,
) -> anyhow::Result<Encoder> {
    let mut encoder = opus::Encoder::new(
        config.config().sample_rate.0,
        opus::Channels::Stereo,
        opus_mode,
    )?;

    encoder.set_bitrate(bitrate)?;
    
    if matches!(opus_mode, opus::Application::Voip) {
        encoder.set_inband_fec(true)?;
    } else {
        encoder.set_inband_fec(false)?;
    }

    Ok(encoder)
}

///
/// Encode raw samples with the [`opus`] encoder
///
/// # Behavior
/// Returns the result of encoding the samples.
/// In the returned result `(usize, Vec<u8>)` the `usize` will indicate the length of the encoded packet.
/// The `Vec<u8>` is the output of the encoding process.
///
/// # Error
/// Returns an error if some kind of error occured during the encoding process.
///
pub fn encode_samples_opus(mut encoder: Encoder, samples: Vec<f32>) -> anyhow::Result<SoundPacket> {
    let mut buffer = vec![];

    let encoded_bytes_count = encoder.encode_float(&samples, &mut buffer)?;

    Ok(SoundPacket {
        encoder_type: EncoderType::Opus(encoder.get_inband_fec()?),
        sample_rate: encoder.get_sample_rate()?,
        channels: 2,
        bytes: buffer,
        bytes_length: encoded_bytes_count as u64,
    })
}

/// Shows the encoder type of the packet.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub enum EncoderType {
    /// The encoder of this packet was [`opus`].
    /// The inner value contains whether.
    Opus(bool),
}
