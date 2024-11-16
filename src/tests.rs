//! Tests help with developing the library by testing various functions.
//! These tests require a speaker, and a microphone to test both input and output.

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use cpal::traits::{DeviceTrait, StreamTrait};
    use tokio::sync::oneshot;

    use crate::io::{self, playback::stream_audio, record::record_audio_with_interrupt};

    #[test]
    fn audio_playback() {
        let host = io::default_host();
        let audio_device = io::get_audio_device(host);

        let output_device = audio_device.output.unwrap();

        let sample_rate = output_device
            .default_output_config()
            .unwrap()
            .sample_rate()
            .0 as f32;

        let mut sample_clock = 0f32;

        let next_value = move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            Some((sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin())
        };

        let err_callback = |err| eprintln!("an error occurred on stream: {}", err);

        let stream =
            stream_audio(output_device, err_callback, std::iter::from_fn(next_value)).unwrap();

        stream.play().unwrap();

        sleep(Duration::from_secs(1));
    }

    #[test]
    fn audio_recording_and_playback() {
        let host = io::default_host();
        let audio_device = io::get_audio_device(host);
        let config = audio_device.get_input_config().unwrap().unwrap();
        let output_device = audio_device.input.unwrap();

        let (sender, receiver) = oneshot::channel::<()>();

        let err_callback = |err| eprintln!("an error occurred on stream: {}", err);

        let buffer_handle =
            record_audio_with_interrupt(receiver, output_device, err_callback, config.into())
                .unwrap();

        sleep(Duration::from_secs(3));

        sender.send(()).unwrap();

        let samples = buffer_handle.lock().clone().into_iter();

        let stream = stream_audio(audio_device.output.unwrap(), err_callback, samples).unwrap();

        stream.play().unwrap();

        sleep(Duration::from_secs(2));
    }
}
