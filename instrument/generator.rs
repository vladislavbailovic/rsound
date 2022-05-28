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

#[derive(Default)]
pub struct Square {
    sine: Sine,
    detune: f32
}

impl Square {
    pub fn new(detune: f32) -> Self {
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
pub struct Triangle {
    detune: f32,
}

impl Triangle {
    pub fn new(detune: f32) -> Self {
        Self{ detune }
    }
}

impl Generator for Triangle {
    fn amplitude_at(&self, point: f32, freq: f32, volume: f32) -> f32 {
        (2.0 * volume / PI) * (freq * point * 2.0 * PI).sin().asin()
    }
}

#[derive(Default)]
pub struct Saw {
    detune: f32
}

impl Saw {
    pub fn new(detune: f32) -> Self {
        Self{ detune }
    }
}

impl Generator for Saw {
    fn amplitude_at(&self, point: f32, freq: f32, volume: f32) -> f32 {
        volume * (2.0 / PI) * (freq * PI * (point % (1.0 / freq)) - (PI / 2.0))
    }
}

pub struct Double {
    one: Triangle,
    two: Triangle
}

impl Double {
    pub fn new() -> Self {
        Double{ one: Triangle::default(), two: Triangle::new(1.033) }
    }
}

impl Generator for Double {
    fn amplitude_at(&self, point: f32, freq: f32, volume: f32) -> f32 {
        self.one.amplitude_at(point, freq, volume * 0.75) +
        self.two.amplitude_at(point, freq / self.two.detune, volume * 0.25)
    }
}
