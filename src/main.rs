use std::io::Cursor;

use console::Term;
use rodio::Sink;
fn main() {
    use rodio::{source::Source, Decoder, OutputStream};
    use std::fs::File;
    use std::io::BufReader;

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    // For now, preload the 5 sound files
    let chant1 = include_bytes!("../samples/chant.wav");
    let chant2 = include_bytes!("../samples/chant2.wav");
    let chant3 = include_bytes!("../samples/chant3.wav");
    let chant4 = include_bytes!("../samples/chantA.wav");
    let chant5 = include_bytes!("../samples/chantC.wav");

    // Load a sound from a file, using a path relative to Cargo.toml
    // Decode that sound file into a source
    // Play the sound directly on the device
    let term = Term::stdout();
    loop {
        // Get a single character from the input, which will be
        // a number between 1 and 5 indicating which sound to play
        let sample = match term.read_char() {
            Ok('1') => &chant1[..],
            Ok('2') => &chant2[..],
            Ok('3') => &chant3[..],
            Ok('4') => &chant4[..],
            Ok('5') => &chant5[..],
            Ok(_) => &chant1[0..0],
            Err(_) => break,
        };
        sink.stop();
        sink.append(Decoder::new(Cursor::new(sample)).unwrap());
        sink.play();
    }
}
