use std::env;
use std::{fs::File, io::Write};

use indicatif::{ProgressBar, ProgressStyle};
use rust_tracer::color::Color;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "image.ppm";
    if args.len() > 1 {
        filename = &args[1];
    }

    let image_width: u32 = 256;
    let image_height: u32 = 256;

    let mut image = File::create(filename).unwrap();

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
            let pixel_color = Color::new(
                0.0,
                y as f32 / (image_height - 1) as f32,
                x as f32 / (image_width - 1) as f32,
            );

            writeln!(&mut image, "{}", pixel_color.to_ppm()).unwrap();
        }
    }
    bar.finish();
}
