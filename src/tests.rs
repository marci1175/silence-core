//!
//!
//!

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use cpal::traits::{DeviceTrait, StreamTrait};

    use crate::io::{self, playback::stream_audio};

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
            (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
        };

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = stream_audio(output_device, err_fn, next_value).unwrap();

        stream.play().unwrap();

        sleep(Duration::from_secs(1));
    }
}
