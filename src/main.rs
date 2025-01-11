use std::io::stdout;
use std::sync::Arc;
use crate::camera::Camera;
use crate::color::{write_color, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rt_weekend::INFINITY;
use crate::sphere::Sphere;
use crate::vec3::{dot, unit_vector, Point3, Vec3};

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

    camera.render(&world);
}
