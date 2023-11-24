mod audio;

use audio::*;

pub use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AudioParams {
    pub frequency: f32,
    pub amplitude: f32,
}

fn main() {
    let audio_params = AudioParams { frequency: 1000.0, amplitude: 0.5 };
    let audio_params = Arc::new(Mutex::new(audio_params));
    start_audio(Arc::clone(&audio_params));
    std::thread::sleep(std::time::Duration::from_millis(2000));
}
