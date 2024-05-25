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
    Pause,
}

fn phrase_one() -> impl Iterator<Item = Sample> + Clone {
    std::iter::once(Sample::One)
}

fn phrase_two() -> impl Iterator<Item = Sample> + Clone {
    std::iter::once(Sample::Two)
}

fn pause() -> impl Iterator<Item = Sample> + Clone {
    std::iter::once(Sample::Pause)
}

fn verse() -> impl Iterator<Item = Sample> + Clone {
    pause().cycle().take(24)
}

fn chant_a() -> impl Iterator<Item = Sample> + Clone {
    phrase_one().cycle().take(3).chain(phrase_two())
}

fn chant_b() -> impl Iterator<Item = Sample> + Clone {
    phrase_one().cycle().take(5)
}

fn chant_c() -> impl Iterator<Item = Sample> + Clone {
    phrase_one().cycle().take(3)
}

fn part_1() -> impl Iterator<Item = Sample> + Clone {
    chant_a().chain(chant_b()).chain(chant_a()).chain(chant_a())
}

fn part_2() -> impl Iterator<Item = Sample> + Clone {
    chant_a().chain(chant_a())
}

fn part_3() -> impl Iterator<Item = Sample> + Clone {
    chant_a()
        .chain(chant_a())
        .chain(chant_a())
        .chain(chant_a())
        .chain(chant_c())
}

fn part_4() -> impl Iterator<Item = Sample> + Clone {
    chant_a().cycle().take(45)
}

fn song() -> impl Iterator<Item = Sample> + Clone {
    pause()
        .chain(part_1())
        .chain(pause())
        .chain(part_2())
        .chain(pause())
        .chain(part_3())
        .chain(pause())
        .chain(part_4())
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
    // A, A
    // Verse
    // A, A, A, A, C
    // break
    // A, A

    let sequence_a = "4445";
    let sequence_b = "4444";
    let sequence_c = "44";
    let song_part_1 = "abaa";
    let song_part_2 = "aa";
    let song_part_3 = "aaac";
    let song_part_4 = "aaaaaaaaa";

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

    // Set up a channel for the key strokes
    let (tx, rx) = channel();
    let (tx_b, rx_b) = channel();
    std::thread::spawn(move || {
        let term = Term::stdout();
        loop {
            let _ = tx.send(term.read_char());
        }
    });

    // Start a synchronization thread...
    std::thread::spawn(move || {
        let mut period = Duration::from_millis(2000);
        let mut auto_beat = false;
        let mut start_time = std::time::Instant::now();
        let epoch_time = std::time::Instant::now();
        let mut beat_times = vec![];
        let mut last_beat = std::time::Instant::now();
        loop {
            // Every 25 msec, we check for a key press
            std::thread::sleep(Duration::from_millis(25));
            if rx.try_recv().is_ok() {
                last_beat = std::time::Instant::now();
                eprintln!("Manual Beat! {:?}", epoch_time.elapsed());
                tx_b.send(BeatType::Manual).unwrap();
                auto_beat = false;
                beat_times.push(epoch_time.elapsed());
                start_time = std::time::Instant::now();
                // if there are at least 4 beat times, we
                // calculate the average time between beats and
                // call this the period.
                if beat_times.len() >= 4 {
                    let beat_diffs = beat_times
                        .windows(2)
                        .map(|w| w[1] - w[0])
                        .collect::<Vec<_>>();
                    let avg_diff = beat_diffs.iter().sum::<Duration>() / beat_diffs.len() as u32;
                    period = avg_diff;
                    eprintln!("Period: {:?}", period);
                    auto_beat = true;
                    beat_times.clear();
                }
            }
            if auto_beat && last_beat.elapsed() >= period {
                // It's been at least half a period since the last beat
                eprintln!("Auto Beat! {:?}", epoch_time.elapsed());
                tx_b.send(BeatType::Auto).unwrap();
                last_beat = std::time::Instant::now();
            }
        }
    });

    // Load a sound from a file, using a path relative to Cargo.toml
    // Decode that sound file into a source
    // Play the sound directly on the device
    let song = song();
    for s in song {
        let mut is_paused = false;
        match s {
            Sample::Pause => {
                eprintln!("Pause here...");
                is_paused = true;
            }
            Sample::One => {
                eprintln!("Sample 1");
                sink.stop();
                sink.append(Decoder::new(Cursor::new(&chant4[..])).unwrap());
                sink.play();
            }
            Sample::Two => {
                eprintln!("Sample 2");
                sink.stop();
                sink.append(Decoder::new(Cursor::new(&chant4[..])).unwrap());
                sink.play();
                std::thread::sleep(Duration::from_millis(1000));
                sink.stop();
                sink.append(Decoder::new(Cursor::new(&chant5[..])).unwrap());
                sink.play();
            }
        }
        if is_paused {
            while rx_b.recv().unwrap() != BeatType::Manual {}
        } else {
            let _ = rx_b.recv().unwrap();
        }
    }
}
