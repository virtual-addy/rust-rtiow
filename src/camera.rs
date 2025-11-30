use std::io::stdout;
use crate::color::{write_color, Color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rt_weekend::{degrees_to_radians, random_f64};
use crate::vec3::{cross, random_in_unit_disk, random_on_hemisphere, random_unit_vector, unit_vector, Point3, Vec3};

pub struct Camera {
    // public
    pub aspect_ratio: f64, // ratio of image width / height
    pub image_width: i32,
    /// rendered image width in pixel count
    pub samples_per_pixel: i32,
    /// count of random samples for each pixel
    pub max_depth: i32,

    pub vfov: i32, // vertical view angle (field of view) in degrees
    /// camera basis vectors
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    //
    pub defocus_angle: f64, // variation angle of rays through each pixel
    pub focus_dist: f64, // distance from camera lookfrom point to plane of perfect focus
    //
    image_height: i32, // rendered image height
    center: Point3, // camera center
    pixel00_loc: Point3, // pixel (0, 0) location
    pixel_delta_u: Vec3, // offset to pixel to the right
    pixel_delta_v: Vec3, // offset to pixel below / downwards
    pixel_samples_scale: f64, // color scale factor for a sum of pixels
    // camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3, // defocus disk horizontal radius
    defocus_disk_v: Vec3, // defocus disk vertical radius
}

impl Camera {
    pub fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,

            vfov: 90,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            //
            defocus_angle: 0.0,
            focus_dist: 10.0,
            //
            image_height: 0,
            center: Point3::zero(),
            pixel00_loc: Point3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            pixel_samples_scale: 1.0,
            u: Vec3::zero(),
            v: Vec3::zero(),
            w: Vec3::zero(),
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
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
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }

                write_color(&mut stdout, &(self.pixel_samples_scale * pixel_color));
            }
        }

        eprintln!("\r Done.            ")
    }

    fn initialize(&mut self) {
        // calculate image height and ensure that it's at least 1
        self.image_height = ((self.image_width as f64) / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        self.center = self.lookfrom;

        // viewport dimensions
        //let focal_length = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.vfov as f64);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * ((self.image_width as f64) / (self.image_height as f64));

        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(self.vup, self.w));
        self.v = cross(self.w, self.u);

        // vector across horizontal and vertical viewport edges
        let viewport_u = viewport_width * self.u; // vector across viewport horizontal edge
        let viewport_v = viewport_height * -self.v; // vector down vertical viewport

        // horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // location of upper left pixel
        let viewport_upper_left = self.center
            - (self.focus_dist * self.w)
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &(dyn Hittable)) -> Color {
        // final case
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        // recursive case
        if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::zero();

            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }

            return Color::new(0.0, 0.0, 0.0);
        }

        // no hits
        let unit_direction = unit_vector(*r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    /// Construct a camera ray originating from origin and directed
    /// at randomly sampled point around pixel location (i,j)
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + (((i as f64) + offset.x()) * self.pixel_delta_u)
            + (((j as f64) + offset.y()) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_f64();

        Ray::timed(ray_origin, ray_direction, ray_time)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    /// Returns a vector in the [-.5,-.5] - [+.5,+.5] unit square
    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_f64() - 0.5, random_f64() - 0.5, 0.0)
    }
}