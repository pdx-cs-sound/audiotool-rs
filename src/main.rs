mod audio;

use audio::*;

fn main() {
    eprintln!("starting");
    let _stream = start_audio();
    std::thread::sleep(std::time::Duration::from_millis(10000));
}
