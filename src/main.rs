#![allow(mixed_script_confusables)]

use std::env;

use rand::prelude::*;
use rust_tracer::material::Material::*;
use rust_tracer::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "image.ppm";
    if args.len() > 1 {
        filename = &args[1];
    }
    let mut c = Camera::new();
    c.aspect_ratio = 16.0 / 9.0;
    c.image_width = 1200;
    c.samples_per_pixel = 500;
    c.max_depth = 50;
    c.vfov = 20.0;
    c.lookfrom = Point::new(13., 2., 3.);
    c.lookat = Point::new(0., 0., 0.);
    c.vup = Vector::new(0., 1., 0.);
    c.defocus_angle = 0.6;
    c.focus_dist = 10.0;

    let mut world = World::new();

    world.add(Sphere(
        Point::new(0., -1000., 0.),
        1000.0,
        Lambertian(Color::new(0.5, 0.5, 0.5)),
    ));
    world.add(Sphere(Point::new(0., 1., 0.), 1.0, Dielectric(1.5)));
    world.add(Sphere(
        Point::new(-4., 1., 0.),
        1.0,
        Lambertian(Color::new(0.1, 0.2, 0.4)),
    ));
    world.add(Sphere(
        Point::new(4., 1., 0.),
        1.0,
        Metal(Color::new(0.7, 0.6, 0.5), 0.1),
    ));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Point::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Point::new(4., 0.2, 0.)).magnitude() > 0.9 {
                let material = if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    Lambertian(albedo)
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_in(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    Metal(albedo, fuzz)
                } else {
                    Dielectric(1.5)
                };

                world.add(Sphere(center, 0.2, material));
            }
        }
    }

    c.render(&world, filename);
}
