use cpal::{StreamConfig, default_host};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::*;

pub fn start_audio(audio_params: Arc<Mutex<AudioParams>>) {
    let host = default_host();
    let device = host.default_output_device()
        .expect("no output device available");
    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    let mut config: StreamConfig = supported_config.into();
    config.channels = 1;
    let sample_rate = config.sample_rate.0 as f32;
    let mut phase = 0usize;
    let mut ticks = 0usize;
    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let guard = audio_params.lock().unwrap();
            let ap: AudioParams = (*guard).clone();
            drop(guard);
            let period = (sample_rate / ap.frequency).floor() as usize;
            let half_period = period / 2;
            if ticks % 100 == 0 {
                eprintln!("{} {} {}", ticks, period, phase);
            }
            ticks += 1;
            for (i, sample) in data.iter_mut().enumerate() {
                let positive = (i + phase) % period > half_period;
                *sample = if positive { ap.amplitude } else { -ap.amplitude };
            }
            phase = (phase + data.len()) % period;
        },
        move |err| {
            eprintln!("audio output error: {:?}", err);
        },
        None // None=blocking, Some(Duration)=timeout
    ).expect("could not build stream");
    stream.play().expect("could not play stream");
}
