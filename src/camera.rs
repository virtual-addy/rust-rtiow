use std::io::stdout;
use crate::color::{write_color, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{unit_vector, Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    // private
    image_height: i32, // rendered image height
    center: Point3, // camera center
    pixel00_loc: Point3, // pixel (0, 0) location
    pixel_delta_u: Vec3, // offset to pixel to the right
    pixel_delta_v: Vec3, // offset to pixel below / downwards
}

impl Camera {
    pub fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        let mut stdout = stdout();

        let image_width = self.image_width;
        let image_height = self.image_height;

        // Render
        println!("P3\n{} {}\n255", image_width, image_height);

        for j in 0..image_height {
            eprintln!("\rScan lines remaining: {}", image_height - j);

            for i in 0..image_width {
                let pixel_center = self.pixel00_loc
                    + ((i as f64) * self.pixel_delta_u)
                    + ((j as f64) * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);

                let pixel_color = self.ray_color(&r, world);
                write_color(&mut stdout, &pixel_color);
            }
        }

        eprintln!("\r Done.            ")
    }

    fn initialize(&mut self) {
        // calculate image height and ensure that it's at least 1
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.center = Point3::new(0.0, 0.0, 0.0);

        // viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        // vector across horizontal and vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // location of upper left pixel
        let viewport_upper_left = self.center
            - Vec3::new(0.0, 0.0, focal_length)
            - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, r: &Ray, world: &(dyn Hittable)) -> Color {
        let mut rec = HitRecord::default();

        if world.hit(r, Interval::new(0.0, f64::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }

        let unit_direction = unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}