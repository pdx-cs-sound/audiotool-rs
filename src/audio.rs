use cpal::platform::{host_from_id, HostId};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{default_host, SampleFormat, Stream, StreamConfig};

use crate::*;

pub struct AudioParams {
    pub frequency: i16,
    pub amplitude: Option<i16>,
    pub sample_rate: f32,
}

impl AudioParams {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            frequency: 69,
            amplitude: None,
            sample_rate,
        }
    }

    pub fn freq_slider_range(&self) -> (i16, i16) {
        let nyquist = self.sample_rate / 2.0;
        let low = 33;
        let high = freq_to_key(0.95 * nyquist);
        (low, high)
    }
}

pub fn key_to_freq(key: i16) -> f32 {
    440.0 * (2.0f32).powf((key as f32 - 69.0) * (1.0 / 12.0))
}

// 440 * 2**((key - 69) / 12) = f
// 2**((key - 69) / 12) = f / 440
// (key - 69) / 12 = log2(f / 440)
// (key - 69) = log2(f / 440) * 12
// key = log2(f / 440) * 12 + 69

pub fn freq_to_key(freq: f32) -> i16 {
    ((freq.log2() - (440.0f32).log2()) * 12.0 + 69.0).round() as i16
}

#[test]
fn test_keys_freqs() {
    let key_freq = [
        (21, 27.5),
        (69, 440.0),
        (108, 4186.0),
    ];

    for (key, freq) in key_freq {
        assert!((key_to_freq(key) - freq).abs() <= 0.1, "ktf {key} {freq}");
        assert_eq!(key, freq_to_key(freq), "ftk {freq} {key}");
    }
}

pub fn db_to_amplitude(d: Option<i16>) -> f32 {
    match d {
        None => 0.0,
        // 20 * log10(a) = d
        // (d / 20) = log10(a)
        // 10**(d / 20) = a
        Some(d) => (10.0f32).powf(d as f32 * (1.0 / 20.0)),
    }
}

#[test]
fn test_db_to_amplitude() {
    assert_eq!(0.0, db_to_amplitude(None));
    assert!((db_to_amplitude(Some(0)) - 1.0).abs() <= 0.01);
}

pub fn key_note_name(key: i16) -> &'static str {
    let note_names = [
        "A",
        "B♭",
        "B",
        "C",
        "D♭",
        "D",
        "E♭",
        "E",
        "F",
        "F♯",
        "G",
        "A♭",
    ];
    let index = (key as usize + 12 - (69 % 12)) % 12;
    note_names[index]
}

#[test]
fn test_key_note_name() {
    let key_names = [
        (31, "G"),
        (69, "A"),
        (84, "C"),
    ];
    for (key, name) in key_names {
        assert_eq!(key_note_name(key), name);
    }
}

pub fn key_note_octave(key: i16) -> i16 {
    key / 12 - 1
}

#[test]
fn test_key_note_octave() {
    let key_octaves = [
        (47, 2),
        (48, 3),
        (71, 4),
        (72, 5),
        (73, 5),
    ];
    for (key, octave) in key_octaves {
        assert_eq!(key_note_octave(key), octave, "{key} {octave}");
    }
}


// Much of this function is borrowed from the `beep`
// example in the CPAL crate.
pub fn start_audio() -> anyhow::Result<(Stream, Arc<Mutex<AudioParams>>)> {
    let host = host_from_id(HostId::Jack).unwrap_or_else(|_| default_host());
    let device = host
        .default_output_device()
        .ok_or(anyhow::anyhow!("could not find default output device"))?;

    let config = device.default_output_config()?;
    assert_eq!(SampleFormat::F32, config.sample_format());
    let config: StreamConfig = config.into();
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let audio_params = AudioParams::new(sample_rate);
    let mut amplitude_was = db_to_amplitude(audio_params.amplitude);
    let mut frequency_was = key_to_freq(audio_params.frequency);
    let params = Arc::new(Mutex::new(audio_params));
    let dup_params = Arc::clone(&params);

    // Produce a block of sinusoids with given parameters.
    let mut sample_clock = 0f32;
    let fill_block = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        use std::f32::consts::PI;

        let ndata = data.len();
        let nsamples = ndata / channels;

        let params = params.lock().unwrap();
        let frequency_is = key_to_freq(params.frequency);
        let amplitude_is = db_to_amplitude(params.amplitude);
        drop(params);

        let da = (amplitude_is - amplitude_was) * 10.0 / sample_rate;
        let mut a = amplitude_was;
        let df = (frequency_is - frequency_was) * 10.0 / sample_rate;
        let mut f = frequency_was;
        for (i, frame) in data.chunks_mut(channels).enumerate() {
            let ts = sample_clock + i as f32;
            let t = ts / sample_rate;
            let y = a * (2.0 * PI * f * t).sin();
            for s in frame {
                *s = y;
            }
            if da > 0.0 && a < amplitude_is || da < 0.0 && a > amplitude_is {
                a += da;
            }
            if df > 0.0 && f < frequency_is || df < 0.0 && f > frequency_is {
                f += df;
            }
        }
        amplitude_was = a;
        frequency_was = f;

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
    Ok((stream, dup_params))
}
