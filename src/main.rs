#![allow(mixed_script_confusables)]

use std::env;
use std::{fs::File, io::Write};

use indicatif::{ProgressBar, ProgressStyle};
use rust_tracer::color::Color;
use rust_tracer::entity::Entity;
use rust_tracer::hittable::Hittable;
use rust_tracer::interval::Interval;
use rust_tracer::ray::Ray;
use rust_tracer::vec3::{Point, Vector};
use rust_tracer::world::World;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "image.ppm";
    if args.len() > 1 {
        filename = &args[1];
    }
    let mut image = File::create(filename).unwrap();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let mut image_height = (image_width as f32 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1
    };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point::new(0., 0., 0.);

    let viewport_u = Vector::new(viewport_width, 0., 0.);
    let viewport_v = Vector::new(0., -viewport_height, 0.);

    let pixelδu = viewport_u / image_width as f32;
    let pixelδv = viewport_v / image_height as f32;

    let viewport_upper_left =
        camera_center - Vector::new(0., 0., focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + (pixelδu + pixelδv) * 0.5;

    let mut world = World::new();
    world.add(Entity::Sphere(Point::new(0., 0., -1.0), 0.5));
    world.add(Entity::Sphere(Point::new(0., -100.5, -1.0), 100.));

    println!("Writing to {}", filename);
    writeln!(&mut image, "P3\n{} {}\n255", image_width, image_height).unwrap();

    let bar = ProgressBar::new((image_width * image_height) as u64).with_style(
        ProgressStyle::with_template(
            "[{elapsed_precise} / {duration_precise}] {bar:50.blue/red} ({percent:>3}%) {pos:>7}/{len:7}",
        )
        .unwrap(),
    );

    for y in 0..image_height {
        for x in 0..image_width {
            let pixel_center = pixel00_loc + pixelδu * x as f32 + pixelδv * y as f32;

            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);

            writeln!(&mut image, "{}", pixel_color.to_ppm()).unwrap();
        }
    }
    bar.finish();
}

fn ray_color(ray: &Ray, world: &World) -> Color {
    let rec = world.hit(ray, &Interval::new(0., f32::INFINITY));
    if let Some(rec) = rec {
        return (rec.normal + Color::white()) * 0.5;
    }
    let unit_direction = ray.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::white() * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}
