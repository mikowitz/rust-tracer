use core::f32;
use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use std::{fs::File, io::Write};

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

    image_height: u32,
    center: Point,
    pixelδu: Vector,
    pixelδv: Vector,
    pixel00_loc: Point,
    pixels_sample_scale: f32,
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
            "[{elapsed_precise} / {eta_precise}] {bar:50.blue/red} ({percent:>3}%) {pos:>7}/{len:7}",
        )
        .unwrap(),
    );

        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let mut pixel_color = Color::black();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color = pixel_color + ray_color(&ray, world);
                }

                pixel_color = pixel_color * self.pixels_sample_scale;

                writeln!(&mut image, "{}", pixel_color.to_ppm()).unwrap();
                bar.inc(1);
            }
        }
        bar.finish();
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let mut rng = rand::thread_rng();
        let x_offset = rng.gen::<f32>() - 0.5;
        let y_offset = rng.gen::<f32>() - 0.5;
        let pixel_sample = self.pixel00_loc
            + self.pixelδu * (x as f32 + x_offset)
            + self.pixelδv * (y as f32 + y_offset);

        let origin = self.center;
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }

    fn initialize(&mut self) {
        let image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        self.image_height = if image_height < 1 { 1 } else { image_height };

        self.pixels_sample_scale = 1.0 / (self.samples_per_pixel as f32);

        self.center = Point::new(0., 0., 0.);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        let viewport_u = Vector::new(viewport_width, 0., 0.);
        let viewport_v = Vector::new(0., -viewport_height, 0.);

        self.pixelδu = viewport_u / self.image_width as f32;
        self.pixelδv = viewport_v / self.image_height as f32;

        let viewport_upper_left =
            self.center - Vector::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + (self.pixelδu + self.pixelδv) * 0.5;
    }
}

fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(rec) = world.hit(ray, &Interval::new(0., f32::INFINITY)) {
        return (rec.normal + Color::white()) * 0.5;
    }
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::white() * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}
