use crate::math::powf::Powf;

pub trait Powi {
    fn powi(&self, exponent: i32) -> f32;
}

impl Powi for f32 {
    fn powi(&self, exponent: i32) -> f32 {
        self.powf(exponent as f32)
    }
}
