mod audio;
mod gui;

pub use std::sync::{Arc, Mutex};

pub use anyhow::*;

pub use audio::*;
pub use gui::*;

fn main() {
    // XXX must hold stream to keep audio playing.
    let (_stream, audio_params) = start_audio().unwrap();
    start_gui(audio_params).unwrap();
}
