mod audio;
mod gui;

pub use std::sync::{Arc, Mutex};

pub use anyhow::*;

pub use audio::*;
pub use gui::*;

fn main() {
    let audio_params = Arc::new(Mutex::new(AudioParams::default()));
    let gui_params = Arc::clone(&audio_params);
    // XXX must hold stream to keep audio playing.
    let _stream = start_audio(audio_params).unwrap();
    start_gui(gui_params).unwrap();
}
