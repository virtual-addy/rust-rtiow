use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn default() -> Self {
        Self {
            orig: Point3::default(),
            dir: Vec3::default(),
        }
    }

    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction
        }
    }

    pub fn origin(&self) -> &Point3 { &self.orig }

    pub fn direction(&self) -> &Vec3 { &self.dir }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}