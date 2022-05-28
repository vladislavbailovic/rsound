pub mod generator;
pub use self::generator::*;

pub mod envelope;
pub use self::envelope::*;

pub trait Generator {
    fn amplitude_at(&self, point: f32, freq: f32, volume: f32) -> f32;
}
