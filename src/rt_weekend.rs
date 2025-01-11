use rand::{random, Rng};

// pub const INFINITY: f64 = f64::INFINITY;
const PI: f64 = 3.1415926535897932385_f64;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// generates a double b/n 0..1
pub fn random_double() -> f64 {
    let mut rng = rand::rng();
    rng.random_range(0.0..1.0)
}

pub fn random_double_within(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}