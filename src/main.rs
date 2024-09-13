#![allow(mixed_script_confusables)]

use std::env;

use rust_tracer::{material::Material, prelude::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "image.ppm";
    if args.len() > 1 {
        filename = &args[1];
    }

    let ground_mat = Material::Lambertian(Color::new(0.8, 0.8, 0.0));
    let center_mat = Material::Lambertian(Color::new(0.1, 0.2, 0.5));
    let left_mat = Material::Metal(Color::new(0.8, 0.8, 0.8), 0.3);
    let right_mat = Material::Metal(Color::new(0.8, 0.6, 0.2), 1.0);
    let mut world = World::new();
    world.add(Sphere(Point::new(0., -100.5, -1.0), 100.0, ground_mat));
    world.add(Sphere(Point::new(0., 0., -1.2), 0.5, center_mat));
    world.add(Sphere(Point::new(-1., 0., -1.0), 0.5, left_mat));
    world.add(Sphere(Point::new(1., 0., -1.0), 0.5, right_mat));

    let mut c = Camera::new();
    c.aspect_ratio = 16.0 / 9.0;
    c.image_width = 1200;
    c.samples_per_pixel = 100;
    c.max_depth = 50;

    c.render(&world, filename);
}
