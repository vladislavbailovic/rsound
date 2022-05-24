use std::fs::File;
use std::io::{Write,BufWriter};
use std::f32::consts::PI;

const SAMPLE_RATE: i32 = 44100;

type Bpm = i32;

fn main() -> std::io::Result<()> {
    use Note::*;
    use Duration::*;
    let mut f = BufWriter::new(File::create("foo.pcm")?);
    // let source = Double::new();
    // let source = Square::default();
    let source1 = Sine::default();
    // let source2 = Triangle::default();
    let source2 = Double::new();
    // let source2 = Saw::default();
    let volume = 0.5;
    let melody = Sequence::new(90, vec![
        H(Quarter, volume),
        H(Quarter, volume),
        H(Quarter, volume),
        Pause(Quarter),
        Fis(Whole, volume),
        E(Half, volume),
        H(Half, volume),
        Fis(Whole, volume),
    ]);
    for sample in melody.play(&source1) {
        f.write(&sample.to_le_bytes())?;
    }
    for sample in melody.play(&source2) {
        f.write(&sample.to_le_bytes())?;
    }
    Ok(())
}

struct Sequence {
    tempo: Bpm,
    sequence: Vec<Note>
}

impl Sequence {
    fn new(tempo: Bpm, sequence: Vec<Note>) -> Self {
        Self{ tempo, sequence }
    }

    fn play(&self, instrument: &impl Generator) -> Vec<f32> {
        let mut score = Vec::new();
        for sound in &self.sequence {
            println!("playing {:?}", sound);
            for sample in sound.signal(self.tempo).play(instrument) {
                score.push(sample);
            }
        }
        score
    }
}

trait Generator {
    fn amplitude_at(&self, point: f32, freq: f32, volume: f32) -> f32;
}

#[derive(Default)]
struct Sine {
    detune: f32
}

impl Sine {
    fn new(detune: f32) -> Self {
        Self{
            detune,
            ..Sine::default()
        }
    }
}

impl Generator for Sine {
    fn amplitude_at(&self, point: f32, freq: f32, volume: f32) -> f32 {
        volume * (freq * point * 2.0 * PI).sin()
    }
}

struct Double {
    one: Sine,
    two: Saw
}

impl Double {
    fn new() -> Self {
        Double{ one: Sine::default(), two: Saw::new(1.3333) }
    }
}

impl Generator for Double {
    fn amplitude_at(&self, point: f32, freq: f32, volume: f32) -> f32 {
        self.one.amplitude_at(point, freq, volume * 0.75) +
        self.two.amplitude_at(point, freq / self.two.detune, volume * 0.25)
    }
}

#[derive(Default)]
struct Square {
    sine: Sine,
    detune: f32
}

impl Square {
    fn new(detune: f32) -> Self {
        Self{
            detune,
            ..Square::default()
        }
    }
}

impl Generator for Square {
    fn amplitude_at(&self, point: f32, freq: f32, volume: f32) -> f32 {
        if self.sine.amplitude_at(point, freq, 1.0) > 0.0 {
            volume
        } else {
            volume * -1.0
        }
    }
}

#[derive(Default)]
struct Triangle {
    detune: f32,
}

impl Triangle {
    fn new(detune: f32) -> Self {
        Self{ detune }
    }
}

impl Generator for Triangle {
    fn amplitude_at(&self, point: f32, freq: f32, volume: f32) -> f32 {
        (2.0 * volume / PI) * (freq * point * 2.0 * PI).sin().asin()
    }
}

#[derive(Default)]
struct Saw {
    detune: f32
}

impl Saw {
    fn new(detune: f32) -> Self {
        Self{ detune }
    }
}

impl Generator for Saw {
    fn amplitude_at(&self, point: f32, freq: f32, volume: f32) -> f32 {
        volume * (2.0 / PI) * (freq * PI * (point % (1.0 / freq)) - (PI / 2.0))
    }
}

struct Signal {
    duration: f32,
    freq: f32,
    volume: f32,
}

impl Signal {
    fn play(&self, generator: &impl Generator) -> Vec<f32> {
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
    Sixteenth,
    Eighth,
    Quarter,
    Half,
    Whole,
}

impl Duration {
    fn length(&self, bpm: Bpm) -> f32 {
        match self {
            Duration::Sixteenth => 60.0 / bpm as f32 / 4.0,
            Duration::Eighth => 60.0 / bpm as f32 / 2.0,
            Duration::Quarter => 60.0 / bpm as f32,
            Duration::Half => (60.0 / bpm as f32) * 2.0,
            Duration::Whole => (60.0 / bpm as f32) * 4.0,
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
    Pause(Duration),
}

impl Note {
    fn signal(&self, bpm: Bpm) -> Signal {
        match self {
            Note::C(d, v) => Signal{duration: d.length(bpm), volume: *v, freq: 261.63},
            Note::Cis(d, v) => Signal{duration: d.length(bpm), volume: *v, freq: 277.18},
            Note::D(d, v) => Signal{duration: d.length(bpm), volume: *v, freq: 293.66},
            Note::Dis(d, v) => Signal{duration: d.length(bpm), volume: *v, freq: 311.13},
            Note::E(d, v) => Signal{duration: d.length(bpm), volume: *v, freq: 329.63},
            Note::F(d, v) => Signal{duration: d.length(bpm), volume: *v, freq: 349.23},
            Note::Fis(d, v) => Signal{duration: d.length(bpm), volume: *v, freq: 369.99},
            Note::G(d, v) => Signal{duration: d.length(bpm), volume: *v, freq: 392.00},
            Note::Gis(d, v) => Signal{duration: d.length(bpm), volume: *v, freq: 415.30},
            Note::A(d, v) => Signal{duration: d.length(bpm), volume: *v, freq: 440.0},
            Note::B(d, v) => Signal{duration: d.length(bpm), volume: *v, freq: 466.16},
            Note::H(d, v) => Signal{duration: d.length(bpm), volume: *v, freq: 493.88},
            Note::Pause(d) => Signal{duration: d.length(bpm), volume: 0.0, freq: 0.0 },
        }
    }
}
