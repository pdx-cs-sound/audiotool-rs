use cpal::platform::{host_from_id, HostId};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{default_host, SampleFormat, Stream, StreamConfig};

use crate::*;

pub struct AudioParams {
    pub frequency: f32,
    pub amplitude: f32,
}

impl Default for AudioParams {
    fn default() -> Self {
        Self {
            frequency: 1000.0,
            amplitude: 0.5,
        }
    }
}

// Much of this function is borrowed from the `beep`
// example in the CPAL crate.
pub fn start_audio(params: Arc<Mutex<AudioParams>>) -> anyhow::Result<Stream> {
    let host = host_from_id(HostId::Jack).unwrap_or_else(|_| default_host());
    let device = host
        .default_output_device()
        .ok_or(anyhow::anyhow!("could not find default output device"))?;

    let config = device.default_output_config()?;
    assert_eq!(SampleFormat::F32, config.sample_format());
    let config: StreamConfig = config.into();
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a block of sinusoids with given parameters.
    let mut sample_clock = 0f32;
    let fill_block = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        use std::f32::consts::PI;

        let params = params.lock().unwrap();
        let f = params.frequency;
        let a = params.amplitude;
        drop(params);

        for (i, frame) in data.chunks_mut(channels).enumerate() {
            let t = sample_clock + i as f32;
            let y = a * (t * f * 2.0 * PI / sample_rate).sin();
            for s in frame {
                *s = y;
            }
        }

        let nsamples = data.len() / channels;
        sample_clock = (sample_clock + nsamples as f32) % sample_rate;
    };

    let stream = device.build_output_stream(
        &config,
        fill_block,
        |err| {
            eprintln!("audio output error: {:?}", err);
        },
        None, // None=blocking, Some(Duration)=timeout
    )?;
    stream.play()?;
    Ok(stream)
}
