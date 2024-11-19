//! Eanbles raw sample encoding to opus.

use opus::{Channels, Encoder};

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
    sample_rate: u32,
    opus_mode: opus::Application,
    bitrate: opus::Bitrate,
    channels: Channels,
) -> anyhow::Result<Encoder> {
    let mut encoder = opus::Encoder::new(sample_rate, channels, opus_mode)?;

    encoder.set_bitrate(bitrate)?;

    if matches!(opus_mode, opus::Application::Voip) {
        encoder.set_inband_fec(true)?;
    } else {
        encoder.set_inband_fec(false)?;
    }

    Ok(encoder)
}

///
/// Encode raw samples with the [`opus`] encoder.
///
/// # Behavior
/// Returns the result of encoding the samples.
/// In the returned result `(usize, Vec<u8>)` the `usize` will indicate the length of the encoded packet.
/// The `Vec<u8>` is the output of the encoding process.
///
/// # Error
/// Returns an error if some kind of error occured during the encoding process.
///
pub fn encode_sample_set_size_opus(
    encoder: &mut Encoder,
    samples: &[f32],
    samples_per_frame: usize,
) -> anyhow::Result<SoundPacket> {
    let mut compressed_buffer = vec![0; 1500];

    let encoded_bytes_count = encoder.encode_float(samples, &mut compressed_buffer)?;

    Ok(SoundPacket {
        encoder_type: EncoderType::Opus(encoder.get_inband_fec()?),
        sample_rate: encoder.get_sample_rate()?,
        channels: 2,
        bytes: compressed_buffer[..encoded_bytes_count].to_vec(),
        samples_per_frame: samples_per_frame as u64,
    })
}

///
/// Encodes raw samples (f32) into a list of [`SoundPacket`]-s.
/// 
/// # Behavior
/// Returns a list of the encoded [`SoundPacket`]-s. The frame duration and the channels ([`Channels`]) is needed to know the [`SoundPacket`]'s size.
/// 
/// # Error
/// Returns an error if the following arguments are invalid:
///     * Invalid raw samples
///     * Invalid frame duration
///
pub fn encode_samples_opus(
    mut encoder: Encoder,
    samples: &[f32],
    frame_duration_ms: u32,
    channels: Channels,
) -> anyhow::Result<Vec<SoundPacket>> {
    let samples_per_frame = (encoder.get_sample_rate()? * frame_duration_ms) / 1000;
    let samples_per_frame = (samples_per_frame * channels as u32) as usize;
    let mut sound_packets = vec![];

    for sample_chunk in samples.chunks(samples_per_frame) {
        let sample = if sample_chunk.len() < samples_per_frame {
            let mut padded_frame = Vec::with_capacity(samples_per_frame);
            padded_frame.extend_from_slice(sample_chunk);
            padded_frame.resize(samples_per_frame, 0.);
            padded_frame
        } else {
            sample_chunk.to_vec()
        };

        let sound_packet = encode_sample_set_size_opus(&mut encoder, &sample, samples_per_frame)?;

        sound_packets.push(sound_packet);
    }

    Ok(sound_packets)
}

/// Shows the encoder type of the packet.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub enum EncoderType {
    /// The encoder of this packet was [`opus`].
    /// The inner value contains whether.
    Opus(bool),
}
