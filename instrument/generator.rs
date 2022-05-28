use std::f32::consts::PI;
use instrument::Generator;

#[derive(Default)]
pub struct Sine {
    detune: f32
}

impl Sine {
    pub fn new(detune: f32) -> Self {
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
    two: Square
}

impl Double {
    fn new() -> Self {
        Double{ one: Sine::default(), two: Square::new(1.33) }
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
