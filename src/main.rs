#![feature(field_init_shorthand)]

extern crate image;
extern crate toml;

mod config;
mod shapes;
mod drawing;

use config::Config;

use image::{ImageBuffer, Rgba};

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

const CONFIG_FILE_PATH: &'static str = "Config.toml";

fn main() {
    println!("Starting...");

    let config = Config::from_config_file(CONFIG_FILE_PATH);
    println!("Config loaded: {:?}", config);

    let initial_image = ImageBuffer::from_pixel(config.size, config.size, image::Rgba([0u8, 0u8, 0u8, 255u8]));

    let (width, height) = initial_image.dimensions();
    println!("Created image with dimensions {:?}x{:?}", width, height);

    let _ = initial_image.save("generated/test.png").unwrap();
}
