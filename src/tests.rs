//! Tests help with developing the library by testing various functions.
//! These tests require a speaker, and a microphone to test both input and output.

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use cpal::{
        traits::{DeviceTrait, StreamTrait},
        StreamError,
    };
    use opus::Decoder;
    use tokio::sync::oneshot;

    use crate::{io::{self, playback::stream_audio, record::record_audio_with_interrupt}, opus::{decode::create_opus_decoder, encode::create_opus_encoder}};

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
        let input_device = audio_device.input.unwrap();

        let samples = record_audio(input_device, config);

        let err_callback = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = stream_audio(
            audio_device.output.unwrap(),
            err_callback,
            samples.into_iter(),
        )
        .unwrap();

        stream.play().unwrap();

        sleep(Duration::from_secs(2));
    }

    #[test]
    fn audio_encoding_decoding() {
        let host = io::default_host();
        let audio_device = io::get_audio_device(host);
        let config = audio_device.get_input_config().unwrap().unwrap();

        let sample = record_audio(audio_device.input.unwrap(), config.clone());

        let mut encoder = create_opus_encoder(config.clone(), opus::Application::Audio, opus::Bitrate::Max).unwrap();

        let mut encoded_buf = vec![];

        encoder.encode_float(&sample, &mut encoded_buf).unwrap();

        let mut decoder = create_opus_decoder(config.sample_rate().0).unwrap();

        let mut decoded_buf = vec![];

        decoder.decode_float(&mut encoded_buf, &mut decoded_buf, false).unwrap();

        let err_callback = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = stream_audio(
            audio_device.output.unwrap(),
            err_callback,
            decoded_buf.into_iter(),
        )
        .unwrap();

        stream.play().unwrap();

        sleep(Duration::from_secs(2));
    }

    fn record_audio(
        input_device: cpal::Device,
        config: cpal::SupportedStreamConfig,
    ) -> Vec<f32> {
        let (sender, receiver) = oneshot::channel::<()>();
        let err_callback = |err| eprintln!("an error occurred on stream: {}", err);

        let buffer_handle =
            record_audio_with_interrupt(receiver, input_device, err_callback, config.into())
                .unwrap();

        sleep(Duration::from_secs(3));

        sender.send(()).unwrap();

        let samples = buffer_handle.lock().clone();
        samples
    }
}
