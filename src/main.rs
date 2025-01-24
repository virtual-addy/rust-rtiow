use std::f64::consts::PI;
use std::io::stdout;
use std::sync::Arc;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian};
use crate::material::Metal;
use crate::sphere::Sphere;
use crate::vec3::{Point3};

mod rt_weekend;
mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod material;

fn main() {
    //world
    let mut world = HittableList::default();

    let R = (PI / 4.0).cos();

    let material_left = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Arc::new(Sphere::new(Point3::new(-R, 0.0, -1.0), R, material_left)));
    world.add(Arc::new(Sphere::new(Point3::new(R, 0.0, -1.0), R, material_right)));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 90;

    camera.render(&world);
}
