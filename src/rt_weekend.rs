
pub const INFINITY: f64 = f64::INFINITY;
const PI: f64 = 3.1415926535897932385_f64;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}