use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::random_unit_vector;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}



