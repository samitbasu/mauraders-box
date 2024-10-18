use std::{io::Cursor, time::Duration};

use console::Term;
use rodio::Sink;
use std::sync::mpsc::channel;

pub enum SongState {
    Part1,
    Part2,
    Part3,
    Part4,
}

pub enum Part1State {
    FirstA(ChantAState),
    SecondB(ChantBState),
    ThirdA(ChantAState),
    FourthA(ChantAState),
}

pub enum Part2State {
    FirstA(ChantAState),
    SecondA(ChantAState),
}

pub enum Part3State {
    FirstA(ChantAState),
    SecondA(ChantAState),
    ThirdA(ChantAState),
    FourthC(ChantCState),
}

pub enum ChantPartState {
    ChantA(ChantAState),
    ChantB(ChantBState),
    ChantC(ChantCState),
}

pub enum ChantAState {
    First4,
    Second4,
    Third4,
    Fourth4,
    Fifth5,
}

pub enum ChantBState {
    First4,
    Second4,
    Third4,
    Fourth4,
}

pub enum ChantCState {
    First4,
    Second4,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Sample {
    One,
    Two,
    Three,
}

fn phrase_one() -> impl Iterator<Item = Sample> + Clone {
    std::iter::once(Sample::One)
}

fn phrase_two() -> impl Iterator<Item = Sample> + Clone {
    std::iter::once(Sample::Two)
}

fn phrase_three() -> impl Iterator<Item = Sample> + Clone {
    std::iter::once(Sample::Three)
}

fn chant_a() -> impl Iterator<Item = Sample> + Clone {
    phrase_one().cycle().take(3).chain(phrase_two())
}

fn chant_b() -> impl Iterator<Item = Sample> + Clone {
    phrase_one().cycle().take(4)
}

fn chant_c() -> impl Iterator<Item = Sample> + Clone {
    phrase_one().cycle().take(3)
}

fn chant_d() -> impl Iterator<Item = Sample> + Clone {
    phrase_one().cycle().take(3).chain(phrase_three())
}

fn part_1() -> impl Iterator<Item = Sample> + Clone {
    chant_a().chain(chant_b()).chain(chant_a()).chain(chant_a())
}

fn part_2() -> impl Iterator<Item = Sample> + Clone {
    chant_d().chain(chant_a())
}

fn part_3() -> impl Iterator<Item = Sample> + Clone {
    chant_d()
        .chain(chant_d())
        .chain(chant_a())
        .chain(chant_a())
        .chain(chant_c())
}

fn part_4() -> impl Iterator<Item = Sample> + Clone {
    chant_a()
        .chain(chant_a())
        .chain(chant_a())
        .chain(chant_a())
        .chain(chant_a())
        .chain(chant_a())
        .chain(chant_a())
        .chain(chant_a())
}

fn song() -> impl Iterator<Item = Sample> + Clone {
    part_1().chain(part_2()).chain(part_3()).chain(part_4())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BeatType {
    Manual,
    Auto,
}

fn main() {
    use rodio::{source::Source, Decoder, OutputStream};
    use std::fs::File;
    use std::io::BufReader;

    // Chant cycle
    // A, B, A, A
    // Verse
    // D A - 11131112
    // Verse
    // D, D, A, A, C - 1113111311121112111
    // A A A A A A A A
    // 4 times through
    // Turn your magic on
    // ...
    // You make me feel
    // 2 times through
    // ...
    // 4.5 times
    // break
    // 9 times through

    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    // For now, preload the 5 sound files
    let chant1 = include_bytes!("../samples/chant.wav");
    let chant2 = include_bytes!("../samples/chant2.wav");
    let chant3 = include_bytes!("../samples/chant3.wav");
    let chant4 = include_bytes!("../samples/chantA.wav");
    let chant5 = include_bytes!("../samples/chantC.wav");
    let chant6 = include_bytes!("../samples/alive-again-chant-vocals.mp3");
    //let lifetime = include_bytes!("../samples/lifetime.wav");

    // Load a sound from a file, using a path relative to Cargo.toml
    // Decode that sound file into a source
    // Play the sound directly on the device
    let song = song();
    let mut song_started = false;
    let mut pause_counter = 0;
    let term = Term::stdout();
    // Get a key to start the song
    /*
    if !song_started {
        eprintln!("Let's go!");
        let sink = Sink::try_new(&stream_handle).unwrap();
        sink.append(Decoder::new(Cursor::new(&lifetime[..])).unwrap());
        sink.detach();
        song_started = true;
    }
    */
    let mut seq1 = part_1().fuse();
    let mut seq2 = part_2().fuse();
    let mut seq3 = part_3().fuse();
    let mut seq4 = part_4().fuse();
    loop {
        let char = term.read_char().unwrap();
        let sample = match char {
            '1' => Some(Sample::One),
            '2' => Some(Sample::Two),
            '3' => Some(Sample::Three),
            _ => break,
        };
        if let Some(sample) = sample {
            match sample {
                Sample::One => {
                    eprintln!("Sample 1");
                    sink.stop();
                    sink.append(Decoder::new(Cursor::new(&chant4[..])).unwrap());
                    sink.play();
                }
                Sample::Two => {
                    eprintln!("Sample 2");
                    sink.stop();
                    sink.append(Decoder::new(Cursor::new(&chant5[..])).unwrap());
                    sink.play();
                }
                Sample::Three => {
                    eprintln!("Sample 3");
                    sink.stop();
                    sink.append(Decoder::new(Cursor::new(&chant6[..])).unwrap());
                    sink.play();
                }
            }
        }
    }
}
