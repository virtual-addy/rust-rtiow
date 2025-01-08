use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{dot, Point3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {

    fn hit(&self, r: &Ray, ray_t_min: f64, ray_t_max: f64, hit_record: &HitRecord) -> bool {
        let oc = *self.center - *r.origin();
        let a = r.direction().length_squared();
        // let b  = -2.0 * dot(r.direction(), &oc);
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - (radius * radius);

        let discriminant = (h * h) - (a * c);

        if discriminant < 0.0 {
            -1.0
        } else {
            (h - discriminant.sqrt()) / a
        }
        todo!()
    }
}