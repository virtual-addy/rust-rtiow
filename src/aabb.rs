use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn default() -> Self {
        Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        }
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: &Point3, b: &Point3) -> Self {
        let x = if a[0] <= b[0] {
            Interval::new(a[0], b[0])
        } else {
            Interval::new(b[0], a[0])
        };

        let y = if a[1] <= b[1] {
            Interval::new(a[1], b[1])
        } else {
            Interval::new(b[1], a[1])
        };

        let z = if a[2] <= b[2] {
            Interval::new(a[2], b[2])
        } else {
            Interval::new(b[2], a[2])
        };

        Self { x, y, z }
    }

    pub  fn axis_interval(&self, n: i32) -> &Interval {
        if n == 1 { return &self.y; }
        if n == 2 { return &self.z; }

        &self.x
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_origin = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let axis_index = axis as usize;
            let ad_inv = 1.0 / ray_dir[axis_index];

            let t0 = (ax.min - ray_origin[axis_index]) * ad_inv;
            let t1 = (ax.max - ray_origin[axis_index]) * ad_inv;

            if t0 < t1 {
                if t0 > ray_t.min { ray_t.min = t0 }
                if t1 > ray_t.max { ray_t.max = t1 }
            }else {
                if t1 > ray_t.min { ray_t.min = t1 }
                if t0 > ray_t.max { ray_t.max = t0 }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }
}
