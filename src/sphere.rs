use crate::color::Color;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::vec3::{dot, Point3, Vec3};
use std::sync::Arc;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn stationary(static_center: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::new(static_center, Vec3::zero()),
            radius: radius.max(0.0),
            mat,
        }
    }

    pub fn moving(center_1: Point3, center_2: Point3, radius: f64, mat: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::new(center_1, center_2 - center_1),
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - *r.origin();

        let a = r.direction().length_squared();
        let h = dot(*r.direction(), oc);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (h * h) - (a * c);

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_d = discriminant.sqrt();

        // nearest root in acceptable range
        let mut root = (h - sqrt_d) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_d) / a;

            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        rec.mat = self.mat.clone();

        true
    }
}
