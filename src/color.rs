use std::io::Write;
use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Color) -> () {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let ir = (255.999 * r) as u32;
    let ig = (255.999 * g) as u32;
    let ib = (255.999 * b) as u32;

    writeln!(out, "{} {} {}", ir, ig, ib).unwrap_or(())
}