#![feature(field_init_shorthand)]

extern crate geo;
extern crate image;
extern crate toml;
extern crate rand;

mod config;
mod drawing;

use config::Config;

use image::{ImageBuffer, Rgba};

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

const CONFIG_FILE_PATH: &'static str = "Config.toml";

fn main() {
    let config = Config::from_config_file(CONFIG_FILE_PATH);
    println!("Config loaded: {:?}", config);

    let initial_image = ImageBuffer::from_pixel(config.width, config.height, image::Rgba([0u8, 0u8, 0u8, 255u8]));

    let (width, height) = initial_image.dimensions();
    println!("Created image with dimensions {:?}x{:?}", width, height);

    let mut image = initial_image;

    for _ in 0..50 {
        let triangle = drawing::random_triangle(width, height);
        let color = drawing::random_color(config.alpha);

        image = drawing::draw_polygon(&image, &triangle, color);
    }

    let _ = image.save("generated/triangle.png").unwrap();
}
