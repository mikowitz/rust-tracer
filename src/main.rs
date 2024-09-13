#![allow(mixed_script_confusables)]

use std::env;

use rust_tracer::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "image.ppm";
    if args.len() > 1 {
        filename = &args[1];
    }

    let mut world = World::new();
    world.add(Entity::Sphere(Point::new(0., 0., -1.0), 0.5));
    world.add(Entity::Sphere(Point::new(0., -100.5, -1.0), 100.));

    let mut c = Camera::new();
    c.aspect_ratio = 16.0 / 9.0;
    c.image_width = 400;
    c.samples_per_pixel = 100;
    c.max_depth = 50;

    c.render(&world, filename);
}
