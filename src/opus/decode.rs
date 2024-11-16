//! Eanbles raw sample decoding from opus

use anyhow::Result;
use opus::Decoder;

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
/// Decodes a sound packet with [`opus`].
///
/// # Behavior
/// The decoder takes `fec` (Forward Error Correction) as an argument.
/// Decodes a sound packet with the [`opus`] decoder into raw samples (Vec<f32>).
///
/// # Error
/// Returns an error if an error occured while decoding the sound packet.
///
pub fn decode_samples_opus(
    mut decoder: Decoder,
    sound_packet_bytes: Vec<u8>,
    fec: bool,
) -> Result<Vec<f32>> {
    let mut buf = vec![];

    decoder.decode_float(&sound_packet_bytes, &mut buf, fec)?;

    Ok(buf)
}
