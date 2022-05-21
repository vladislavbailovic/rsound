use std::fs::File;
use std::io::{Write,BufWriter};
use std::f32::consts::PI;

const SAMPLE_RATE: i32 = 44100;

type Bpm = i32;

fn main() -> std::io::Result<()> {
    let mut f = BufWriter::new(File::create("foo.pcm")?);
    let source = Generator{};
    let volume = 0.2;
    let melody = vec![
        Note::H(Duration::Whole(90), volume),
        Note::F(Duration::Whole(90), volume),
        Note::E(Duration::Half(90), volume),
        Note::H(Duration::Half(90), volume),
        Note::F(Duration::Whole(90), volume),
    ];
    for note in melody {
        println!("playing {:?}", note);
        for sample in note.tone().play(&source) {
            f.write(&sample.to_le_bytes())?;
        }
    }
    Ok(())
}

struct Generator {
}

impl Generator {
    fn amplitude_at(&self, sample: f32, freq: f32, volume: f32) -> f32 {
        volume * (freq * sample * 2.0 * PI).sin()
    }
}

struct Tone {
    duration: f32,
    freq: f32,
    volume: f32,
}

impl Tone {
    fn play(&self, generator: &Generator) -> Vec<f32> {
        let mut samples: Vec<f32> = Vec::new();
        let duration = (SAMPLE_RATE as f32 * self.duration).floor() as i32;
        for i in 0..duration {
            let t = i as f32 / SAMPLE_RATE as f32;
            samples.push(generator.amplitude_at(t, self.freq, self.volume));
        }
        samples
    }
}

#[derive(Debug)]
enum Duration {
    Sixteenth(Bpm),
    Eighth(Bpm),
    Quarter(Bpm),
    Half(Bpm),
    Whole(Bpm),
}

impl Duration {
    fn length(&self) -> f32 {
        match self {
            Duration::Sixteenth(bpm) => 60.0 / *bpm as f32 / 4.0,
            Duration::Eighth(bpm) => 60.0 / *bpm as f32 / 2.0,
            Duration::Quarter(bpm) => 60.0 / *bpm as f32,
            Duration::Half(bpm) => (60.0 / *bpm as f32) * 2.0,
            Duration::Whole(bpm) => (60.0 / *bpm as f32) * 4.0,
        }
    }
}

type Volume = f32;

#[derive(Debug)]
enum Note {
    C(Duration, Volume),
    Cis(Duration, Volume),
    D(Duration, Volume),
    Dis(Duration, Volume),
    E(Duration, Volume),
    F(Duration, Volume),
    Fis(Duration, Volume),
    G(Duration, Volume),
    Gis(Duration, Volume),
    A(Duration, Volume),
    B(Duration, Volume),
    H(Duration, Volume),
}

impl Note {
    fn tone(&self) -> Tone {
        match self {
            Note::C(d, v) => Tone{duration: d.length(), volume: *v, freq: 261.63},
            Note::Cis(d, v) => Tone{duration: d.length(), volume: *v, freq: 277.18},
            Note::D(d, v) => Tone{duration: d.length(), volume: *v, freq: 293.66},
            Note::Dis(d, v) => Tone{duration: d.length(), volume: *v, freq: 311.13},
            Note::E(d, v) => Tone{duration: d.length(), volume: *v, freq: 329.63},
            Note::F(d, v) => Tone{duration: d.length(), volume: *v, freq: 349.23},
            Note::Fis(d, v) => Tone{duration: d.length(), volume: *v, freq: 369.99},
            Note::G(d, v) => Tone{duration: d.length(), volume: *v, freq: 392.00},
            Note::Gis(d, v) => Tone{duration: d.length(), volume: *v, freq: 415.30},
            Note::A(d, v) => Tone{duration: d.length(), volume: *v, freq: 440.0},
            Note::B(d, v) => Tone{duration: d.length(), volume: *v, freq: 466.16},
            Note::H(d, v) => Tone{duration: d.length(), volume: *v, freq: 493.88},
        }
    }
}
