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
use crate::vec3::{Point3, Vec3};

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

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let material_bubble = Arc::new(Dielectric::new(1.0/1.50));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Arc::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Arc::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.vfov = 20;
    camera.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    camera.lookat = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;

    camera.render(&world);
}
