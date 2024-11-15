//! The crate offers playback capabilities via being a middleware on [`cpal`].

use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, StreamTrait}, BufferSize, Sample, SizedSample, Stream, StreamConfig, StreamError
};

use super::OutputDevice;

/// # Information
/// Plays back audio from a `callback` to an [`OutputDevice`].
/// 
/// # Behavior
/// The [`Stream`] returned by this function will not play automaticly, you will have to call [`Stream::play`] to start playing.
/// If the ongoing [`Stream`] is dropped the audio stream will stop.
/// 
/// # Error
/// The `error_callback` is called when an error occurs while streaming to the output.
pub fn stream_audio<
    //This is the type of the `Sample`-s we are streaming to the [`OutputDevice`]
    T: SizedSample + Send + Sync + cpal::FromSample<f32> + 'static,
    //Error callback is called when the output_stream encounters an error
    E: FnMut(StreamError) + Send + 'static,
>(
    device: OutputDevice,
    error_callback: E,
    mut sample_callback: impl FnMut() -> T + Send + 'static,
) -> Result<Stream> {
    let supported_config = device.default_output_config()?;
    let channel_count = supported_config.channels();

    let stream = device.build_output_stream(
        &StreamConfig {
            channels: channel_count,
            sample_rate: supported_config.sample_rate(),
            buffer_size: {
                match supported_config.buffer_size() {
                    cpal::SupportedBufferSize::Range { min, max: _ } => BufferSize::Fixed(*min),
                    cpal::SupportedBufferSize::Unknown => BufferSize::Default,
                }
            },
        },
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channel_count as usize, &mut sample_callback)
        },
        error_callback,
        None,
    )?;

    Ok(stream)
}

/// # Information
/// Writes data to the output buffer from calling the `sample_callback`
/// # Error
/// This function cannot panic in itself.
fn write_data<T>(output: &mut [T], channels: usize, sample_callback: &mut dyn FnMut() -> T)
where
    T: Sample + cpal::FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = sample_callback().to_sample();
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
