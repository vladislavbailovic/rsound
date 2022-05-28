#[derive(Default)]
pub struct Envelope {
    attack: Option<f32>,
    sustain: Option<f32>,
    release: Option<f32>
}

impl Envelope {
    pub fn new(attack: Option<f32>, sustain: Option<f32>, release: Option<f32>) -> Self {
        Self{ attack, sustain, release }
    }

    pub fn amplitude_at(&self, point: f32, volume: f32, duration: f32) -> f32 {
        if let Some(attack) = self.attack {
            if attack > point {
                let res = volume * (point/attack);
                return res;
            }
        }
        if let Some(sustain) = self.sustain {
            let ms = duration - sustain;
            if ms > point {
                return volume;
            }
        }
        if let Some(release) = self.release {
            let minr = duration - release;
            if point > minr {
                let posr = duration - point;
                let res = volume * (posr/release);
                return res;
            }
        }
        volume
    }
}

