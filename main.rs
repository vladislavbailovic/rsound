mod output;
use output::{pcm,ppm};

mod instrument;
use instrument::*;

const SAMPLE_RATE: i32 = 44100;

type Bpm = i32;

fn draw_sample_envelope() -> std::io::Result<()> {
    let duration = 100.0/1000.0;
    let envelope = Envelope::new(Some(20.0/1000.0), None, Some(50.0/1000.0));
    let graph = EnvelopeGraph::new(&envelope, duration);
    graph.save("foo.ppm")
}

struct EnvelopeGraph {
    graph: ppm::Graph
}

impl EnvelopeGraph {
    pub fn new(envelope: &Envelope, over: f32) -> Self {
        let mut graph = ppm::Graph::new(&EnvelopeGraph::extract_samples(&envelope, over));
        graph.align(ppm::Align::Bottom).snap(ppm::Snap::Width);
        Self {
            graph,
        }
    }

    pub fn save(&self, name: &str) -> std::io::Result<()> {
        self.graph.save(name)
    }

    fn extract_samples(envelope: &Envelope, duration: f32) -> Vec<f32> {
        let mut samples = Vec::new();
        let all = (duration * 1000.0).floor() as i32;
        for x in 1..all {
            let sample = envelope.amplitude_at(x as f32/1000.0, 1.0, duration);
            samples.push(sample*-1.0);
        }
        samples
    }
}

fn main() -> std::io::Result<()> {
    draw_sample_envelope();
    return Ok(());
    use Note::*;
    use Duration::*;
    // let source = Sine::default();
    // let source = Square::default();
    // let source = Triangle::default();
    // let source = Saw::default();
    let source = Double::new();
    let volume = 0.5;
    let envelope = Envelope::new(Some(0.02), None, Some(0.04));
    /*
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
    */
    let melody = Sequence::new(133, vec![
        A(Sixteenth, volume),
        A(Sixteenth, volume),
        A(Sixteenth, volume),
        Pause(Sixteenth),
        A(Eighth, volume),
        G(Eighth, volume),
        Fis(Quarter, volume),
        Fis(Eighth, volume),
        Pause(Twentyfour),
        Fis(Quarter, volume),

        Fis(Sixteenth, volume),
        Fis(Sixteenth, volume),
        G(Sixteenth, volume),
        Pause(Sixteenth),
        G(Eighth, volume),
        A(Quarter, volume),
        A(Eighth, volume),
        Pause(Thirtytwo),
        A(Eighth, volume),

        A(Sixteenth, volume),
        A(Sixteenth, volume),
        A(Sixteenth, volume),
        Pause(Sixteenth),
        A(Eighth, volume),
        G(Eighth, volume),
        Fis(Quarter, volume),
        Fis(Eighth, volume),
        Pause(Twentyfour),
        Fis(Quarter, volume),

        Fis(Sixteenth, volume),
        G(Sixteenth, volume),
        A(Sixteenth, volume),
        Pause(Sixteenth),
        G(Eighth, volume),
        Fis(Quarter, volume),
        Fis(Eighth, volume),
        Pause(Twentyfour),
        Fis(Eighth, volume),
    ]);
    let mut samples = Vec::new();
    for sample in melody.play(&source, &envelope) {
        samples.push(sample);
    }
    ppm::save(&samples)?;
    pcm::save(&samples)?;

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

    fn play(&self, instrument: &impl Generator, envelope: &Envelope) -> Vec<f32> {
        let mut score = Vec::new();
        for sound in &self.sequence {
            println!("playing {:?}", sound);
            for sample in sound.signal(self.tempo).play(instrument, &envelope) {
                score.push(sample);
            }
        }
        score
    }
}

struct Signal {
    duration: f32,
    freq: f32,
    volume: f32,
}

impl Signal {
    fn play(&self, generator: &impl Generator, envelope: &Envelope) -> Vec<f32> {
        let mut samples: Vec<f32> = Vec::new();
        let duration = (SAMPLE_RATE as f32 * self.duration).floor() as i32;
        for i in 0..duration {
            let t = i as f32 / SAMPLE_RATE as f32;
            let volume = envelope.amplitude_at(t, self.volume, self.duration);
            samples.push(generator.amplitude_at(t, self.freq, volume));
        }
        samples
    }
}

#[derive(Debug)]
enum Duration {
    Thirtytwo,
    Twentyfour,
    Sixteenth,
    Eighth,
    Quarter,
    Half,
    Whole,
}

impl Duration {
    fn length(&self, bpm: Bpm) -> f32 {
        match self {
            Duration::Thirtytwo => 60.0 / bpm as f32 / 8.0,
            Duration::Twentyfour => 60.0 / bpm as f32 / 6.0,
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
