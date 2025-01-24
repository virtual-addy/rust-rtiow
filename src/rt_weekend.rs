use rand::Rng;

const PI: f64 = 3.1415926535897932385_f64;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Returns a random real number in [0,1)
pub fn random_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random_range(0.0..1.0)
}

/// Returns a random real number in [min,max)
pub fn random_f64_within(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}