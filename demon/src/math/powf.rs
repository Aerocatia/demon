pub trait Powf {
    fn powf(&self, exponent: f32) -> f32;
}

impl Powf for f32 {
    fn powf(&self, exponent: f32) -> f32 {
        libm::powf(*self, exponent)
    }
}
