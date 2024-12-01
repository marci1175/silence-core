//! Tests help with developing the library by testing various functions.
//! These tests require a speaker, and a microphone to test both input and output.

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, fs, thread::sleep, time::Duration};

    use cpal::traits::{DeviceTrait, StreamTrait};
    use deepsize::DeepSizeOf;
    use opencv::videoio::{CAP_ANY, CAP_MSMF};
    use opus::Channels;
    use ravif::Encoder;

    use tokio::sync::oneshot;
    use crate::{
        avif::encoding::encode_raw_image,
        cam,
        io::{self, playback::stream_audio, record::record_audio_with_interrupt},
        opus::{
            decode::{create_opus_decoder, decode_samples_opus},
            encode::{create_opus_encoder, encode_samples_opus},
        },
    };

    #[test]
    fn image_encode() {
        let mut webcam = cam::Webcam::new_def(CAP_ANY).unwrap();
        let (bytes, size) = webcam.get_frame().unwrap();

        let encoder = Encoder::new().with_speed(3);
        let encoded = encode_raw_image(
            encoder.clone(),
            &bytes,
            size.width as usize,
            size.height as usize,
        )
        .unwrap();

        assert_ne!(dbg!(encoded.avif_file.len()), 0)
    }

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

    fn record_audio(input_device: cpal::Device, config: cpal::SupportedStreamConfig) -> VecDeque<f32> {
        let (sender, receiver) = oneshot::channel::<()>();
        let err_callback = |err| eprintln!("an error occurred on stream: {}", err);

        let buffer_handle =
            record_audio_with_interrupt(input_device, receiver, err_callback, config.into())
                .unwrap();

        sleep(Duration::from_millis(30));

        sender.send(()).unwrap();

        let samples = buffer_handle.lock().clone();
        samples
    }

    #[test]
    fn audio_encoding_decoding() {
        let host = io::default_host();
        let audio_device = io::get_audio_device(host);
        let config = audio_device.get_input_config().unwrap().unwrap();

        let channels = Channels::Stereo;

        let sample = record_audio(audio_device.input.unwrap(), config.clone());

        let encoder = create_opus_encoder(
            48000,
            opus::Application::Audio,
            opus::Bitrate::Max,
            opus::Channels::Stereo,
        )
        .unwrap();

        let sound_packets: Vec<crate::io::SoundPacket> = encode_samples_opus(encoder, &Into::<Vec<f32>>::into(sample), 20, channels).unwrap();

        let decoder = create_opus_decoder(48000).unwrap();

        dbg!(sound_packets.deep_size_of());

        let decoded_buf = decode_samples_opus(decoder, sound_packets).unwrap();

        let err_callback = |err| eprintln!("an error occurred on stream: {}", err);

        sleep(Duration::from_secs(1));

        let stream = stream_audio(
            audio_device.output.unwrap(),
            err_callback,
            decoded_buf.into_iter(),
        )
        .unwrap();

        stream.play().unwrap();

        sleep(Duration::from_secs(3));
    }
}
