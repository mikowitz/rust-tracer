use core::f32;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use std::{f32::consts::PI, fs::File, io::Write};

use crate::{
    color::Color,
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    vec3::{Point, Vector},
    world::World,
};

#[derive(Default)]
pub struct Camera {
    pub image_width: u32,
    pub aspect_ratio: f32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f32,
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Vector,
    pub defocus_angle: f32,
    pub focus_dist: f32,

    image_height: u32,
    center: Point,
    pixelδu: Vector,
    pixelδv: Vector,
    pixel00_loc: Point,
    pixels_sample_scale: f32,
    defocus_disk_u: Vector,
    defocus_disk_v: Vector,
}

impl Camera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&mut self, world: &World, filename: &str) {
        self.initialize();

        let mut image = File::create(filename).unwrap();

        println!("Writing to {}", filename);
        writeln!(
            &mut image,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )
        .unwrap();

        let bar = ProgressBar::new((self.image_width * self.image_height) as u64).with_style(
            ProgressStyle::with_template(
                "[{elapsed_precise} / {eta_precise}] {bar:50.blue/red} ({percent:>3}%) {pos:>6}/{len:6} ({per_sec})",
            )
            .unwrap(),
        );

        let mut coords: Vec<(u32, u32)> = vec![];
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                coords.push((x, y));
            }
        }

        let rows: Vec<&mut [(u32, u32)]> = coords.chunks_mut(self.image_width as usize).collect();

        let pixels = rows
            .into_par_iter()
            .map(|row| self.generate_row(row, world, &bar))
            .flatten()
            .collect::<Vec<String>>()
            .join("\n");

        bar.finish();
        writeln!(&mut image, "{}", pixels).unwrap();
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let mut rng = rand::thread_rng();
        let x_offset = rng.gen::<f32>() - 0.5;
        let y_offset = rng.gen::<f32>() - 0.5;
        let pixel_sample = self.pixel00_loc
            + self.pixelδu * (x as f32 + x_offset)
            + self.pixelδv * (y as f32 + y_offset);

        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let direction = pixel_sample - origin;

        Ray::new(origin, direction, rng.gen::<f32>())
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = Point::random_in_unit_disk();
        self.center + self.defocus_disk_u * p.x + self.defocus_disk_v * p.y
    }

    fn initialize(&mut self) {
        let image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        self.image_height = if image_height < 1 { 1 } else { image_height };

        self.pixels_sample_scale = 1.0 / (self.samples_per_pixel as f32);

        self.center = self.lookfrom;

        let θ = degrees_to_radians(self.vfov);
        let h = (θ / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        let w = (self.lookfrom - self.lookat).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        self.pixelδu = viewport_u / self.image_width as f32;
        self.pixelδv = viewport_v / self.image_height as f32;

        let viewport_upper_left =
            self.center - w * self.focus_dist - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + (self.pixelδu + self.pixelδv) * 0.5;

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
    }

    fn generate_row(
        &self,
        row: &mut [(u32, u32)],
        world: &World,
        bar: &ProgressBar,
    ) -> Vec<String> {
        row.into_par_iter()
            .map(|c| self.generate_pixel(c.0, c.1, world, bar))
            .collect::<Vec<String>>()
    }

    fn generate_pixel(&self, x: u32, y: u32, world: &World, bar: &ProgressBar) -> String {
        let pixel_color: Color = (0..self.samples_per_pixel)
            .into_par_iter()
            .map(|_| {
                let ray = self.get_ray(x, y);
                ray_color(&ray, self.max_depth, world)
            })
            .reduce(Color::black, |a, b| a + b)
            * self.pixels_sample_scale;
        bar.inc(1);
        pixel_color.to_ppm()
    }
}

fn ray_color(ray: &Ray, depth: u32, world: &World) -> Color {
    if depth == 0 {
        return Color::black();
    }

    if let Some(rec) = world.hit(ray, &Interval::new(0.001, f32::INFINITY)) {
        if let Some(scatter) = rec.material.scatter(ray, &rec) {
            return scatter.attenuation * ray_color(&scatter.scattered, depth - 1, world);
        } else {
            return Color::black();
        }
    }
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::white() * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}
