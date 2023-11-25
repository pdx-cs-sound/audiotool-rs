mod audio;

pub use std::sync::{Arc, Mutex};

pub use anyhow::*;

use audio::*;

fn main() {
    eprintln!("starting");
    let params = Arc::new(Mutex::new(AudioParams::default()));
    let _stream = start_audio(params).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(10000));
}
