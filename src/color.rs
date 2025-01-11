use std::io::Write;
use crate::interval::Interval;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> () {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let intensity  = Interval::new(0.000, 0.999);
    let ir = (256.0 * intensity.clamp(r)) as u32;
    let ig = (256.0 * intensity.clamp(g)) as u32;
    let ib = (256.0 * intensity.clamp(b)) as u32;

    writeln!(out, "{} {} {}", ir, ig, ib).unwrap_or(())
}