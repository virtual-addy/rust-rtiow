use crate::vec3::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
    tm: f64,
}

impl Ray {
    pub fn default() -> Self {
        Self {
            orig: Point3::zero(),
            dir: Vec3::zero(),
            tm: 0.0,
        }
    }

    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
            tm: 0.0,
        }
    }

    pub fn timed(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            orig: origin,
            dir: direction,
            tm: time,
        }
    }

    pub fn origin(&self) -> &Point3 { &self.orig }

    pub fn direction(&self) -> &Vec3 { &self.dir }

    pub fn time(&self) -> f64 { self.tm }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}