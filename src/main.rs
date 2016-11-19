#![feature(plugin)]
#![plugin(clippy)]

#![feature(field_init_shorthand)]

extern crate geo;
extern crate image;
extern crate toml;
extern crate rand;

mod config;
mod drawing;
mod genetic;
mod image_description;

use std::env;

use config::Config;
use drawing::ColoredPolygon;
use image_description::{ImageDescription, mutate};

const CONFIG_FILE_PATH: &'static str = "Config.toml";

fn main() {
    let image_file = env::args().nth(1).expect("No image file was provided");
    let input_image = image::open(image_file).unwrap().to_rgba();
    let (width, height) = input_image.dimensions();
    println!("Image loaded: {}x{}", width, height);

    let config = Config::from_config_file(CONFIG_FILE_PATH, width, height);
    println!("Config loaded: {:?}", config);

    let mut polygons: Vec<ColoredPolygon> = vec![];
    polygons.push(drawing::random_colored_triangle(config.width, config.height, config.alpha));

    let initial_image_desc = ImageDescription {
        width: config.width,
        height: config.height,
        polygons: polygons
    };

    let mut image_desc = initial_image_desc;
    let mut last_error = genetic::fitness(&input_image, &image_desc);

    for i in 0..100000 {
        let new_candidate = mutate(&image_desc, &config);
        let new_error = genetic::fitness(&input_image, &new_candidate);

        if new_error < last_error {
            println!("{}: Decreased error from {} to {}", i, last_error, new_error);

            image_desc = new_candidate;
            last_error = new_error;

            let img = drawing::draw_image(&image_desc);
            img.save(format!("generated/{}.png", i)).unwrap();
        }
    }

    let final_image = drawing::draw_image(&image_desc);
    final_image.save("generated/after.png").unwrap();
}
