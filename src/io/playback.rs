//! The crate offers playback capabilities via being a middleware on [`cpal`].

use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, StreamTrait},
    BufferSize, SizedSample, Stream, StreamConfig, StreamError,
};

use super::OutputDevice;

///
/// Plays back audio from an [`Iterator`] to an [`OutputDevice`].
///
/// # Behavior
/// The [`Stream`] returned by this function will not play automaticly, you will have to call [`StreamTrait::play`] to start playing.
/// If the ongoing [`Stream`] is dropped the audio stream will stop.
///
/// # Error
/// The `error_callback` is called when an error occurs while streaming to the output.
/// 
pub fn stream_audio<
    //This is the type of the `Sample`-s we are streaming to the [`OutputDevice`]
    T: SizedSample + Send + Sync + cpal::FromSample<f32> + 'static,
    //Error callback is called when the output_stream encounters an error
    E: FnMut(StreamError) + Send + 'static,
    //The iterator for writing the samples to the output / data buffer
    S: Iterator<Item = T> + Send + 'static + Clone,
>(
    device: OutputDevice,
    error_callback: E,
    mut samples: S,
) -> Result<Stream> {
    //Get supported config
    let supported_config = device.default_output_config()?;

    //Get cahnnel count
    let channel_count = supported_config.channels();

    //Create data `Stream` and return it
    let stream: Stream = device.build_output_stream(
        //Get the `StreamConfig` from the default input and output devices.
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
        //Data writer callback, write the samples to the frames through this
        move |data: &mut [T], _info: &cpal::OutputCallbackInfo| {
            //Write the samples to the data buffer
            for frame in data {
                *frame = if let Some(sample) = samples.next() {
                    //Write the sample to the frame
                    sample
                } else {
                    //If there arent any samples left, write silence
                    T::from_sample(0.0)
                };
            }
        },
        //If an error occurs while writing the data this function will be called
        error_callback,
        //Timeout
        None,
    )?;

    //Return the `Stream` handle
    Ok(stream)
}
