use std::io::BufReader;
pub fn play(filename: &str) {
    let device = rodio::default_output_device().expect("device");
    let sink = rodio::Sink::new(&device);

    let file = std::fs::File::open(filename).expect("open");
    sink.append(rodio::Decoder::new(BufReader::new(file)).expect("decoder"));

    sink.sleep_until_end();
}
