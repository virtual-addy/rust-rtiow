use std::io::stdout;
use std::sync::Arc;
use crate::camera::Camera;
use crate::hittable::{ Hittable};
use crate::hittable_list::HittableList;
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

fn main() {
    //world
    let mut world = HittableList::default();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::default();
    camera.aspect_ratio  = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
}
