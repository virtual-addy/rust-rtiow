use std::io::Write;
use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);
}


pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }

    0.0
}

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> () {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // apply linear to gamma transform for gamma 2
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    let intensity = Interval::new(0.000, 0.999);
    let ir = (256.0 * intensity.clamp(r)) as u8;
    let ig = (256.0 * intensity.clamp(g)) as u8;
    let ib = (256.0 * intensity.clamp(b)) as u8;

    writeln!(out, "{} {} {}", ir, ig, ib).unwrap_or(())
}