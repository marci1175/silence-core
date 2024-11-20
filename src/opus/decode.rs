//! Eanbles raw sample decoding from opus.

use anyhow::Result;
use opus::Decoder;

use super::SoundPacket;

///
/// Create an [`opus`] decoder.
///
/// # Behavior
/// Creates an [`opus`] decoder from a specified sample rate (`u32`).
///
/// # Error
/// Returns an error when created with an invalid sample rate.
///
pub fn create_opus_decoder(sample_rate: u32) -> anyhow::Result<Decoder> {
    let decoder: Decoder = Decoder::new(sample_rate, opus::Channels::Stereo)?;

    Ok(decoder)
}

///
/// Decodes a [`SoundPacket`] (encoded with the [`opus`] codec), into raw samples.
///
/// # Behavior
/// The decoder takes `fec` (Forward Error Correction) as an argument.
/// Decodes a sound packet with the [`opus`] decoder into raw samples (`Vec<f32>`).
/// All additional information is included in the [`SoundPacket`] to maximise code efficiency.
///
/// # Error
/// Returns an error if an error occured while decoding the sound packet.
///
pub fn decode_sample_set_size_opus(
    decoder: &mut Decoder,
    sound_packet: SoundPacket,
    fec: bool,
) -> Result<Vec<f32>> {
    let mut buf = vec![0f32; sound_packet.samples_per_frame as usize];

    decoder.decode_float(&sound_packet.bytes, &mut buf, fec)?;

    Ok(buf)
}

///
/// Decodes a list of [`SoundPacket`]-s, into one raw sample.
///
/// # Behavior
/// The function takes a [`Decoder`] and a list of [`SoundPacket`]-s to decode. All information about the decoding process is included in said [`SoundPacket`]-s.
///
/// # Error
/// Returns an error, if the [`SoundPacket`] is corrupted (Contains invalid data)
///
pub fn decode_samples_opus(
    mut decoder: Decoder,
    sound_packets: Vec<SoundPacket>,
) -> anyhow::Result<Vec<f32>> {
    let mut samples = vec![];

    for sound_packet in sound_packets {
        let super::encode::EncoderType::Opus(fec) = sound_packet.encoder_type;

        let decoded_samples = decode_sample_set_size_opus(&mut decoder, sound_packet, fec)?;

        samples.extend(decoded_samples);
    }

    Ok(samples)
}
