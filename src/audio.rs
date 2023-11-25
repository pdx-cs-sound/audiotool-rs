use cpal::{Stream, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::platform::{HostId, host_from_id};

pub fn start_audio() -> Stream {
    eprintln!("starting audio");
    let host = host_from_id(HostId::Jack).expect("no jack");
    let device = host.default_output_device()
        .expect("no output device available");
    eprintln!("{}", device.name().unwrap());

    let default_config = device.default_output_config().unwrap();
    println!("Default config: {:?}", default_config);
    let derived_config: StreamConfig = default_config.into();
    println!("Derived config: {:?}", derived_config);
    /*
    let config = StreamConfig {
        channels: 1,
        sample_rate: SampleRate(48000),
        buffer_size: BufferSize::Fixed(1024),
    };
    println!("Constructed config: {:?}", config);
    */
    let config = derived_config;
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let stream = device.build_output_stream(
        &config,
        move |data: &mut[f32], _: &cpal::OutputCallbackInfo| {
            for frame in data.chunks_mut(channels) {
                let value = next_value();
                for sample in frame {
                    *sample = value;
                }
            }
        },
        |err| {
            eprintln!("audio output error: {:?}", err);
        },
        None, // None=blocking, Some(Duration)=timeout
    ).expect("could not build stream");
    eprintln!("starting stream");
    stream.play().expect("could not play stream");
    stream
}
