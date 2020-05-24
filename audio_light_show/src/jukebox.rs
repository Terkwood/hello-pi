use std::io::BufReader;
pub fn play(filename: &str) {
    let device = rodio::default_output_device().expect("device");

    let file = std::fs::File::open(filename).expect("file");
    rodio::play_once(&device, BufReader::new(file)).expect("play");
}
