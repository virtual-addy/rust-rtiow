use std::cmp::max;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{dot, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self{ center, radius: radius.max(0.0) }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t_min: f64, ray_t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (h * h) - (a * c);

        if discriminant < 0.0 { return false; }

        let sqrt_d = discriminant.sqrt();

        // nearest root in acceptable range
        let mut root = (h - sqrt_d) / a;
        if root <= ray_t_min || ray_t_max <= root {
            root = (h + sqrt_d) / a;

            if root <= ray_t_min || ray_t_max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);

        true
    }
}