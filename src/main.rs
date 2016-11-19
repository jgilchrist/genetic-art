#![feature(plugin)]
#![plugin(clippy)]

#![feature(field_init_shorthand)]

extern crate geo;
extern crate image;
extern crate toml;
extern crate rand;

mod config;
mod drawing;
mod image_description;

use config::Config;
use drawing::ColoredPolygon;
use image_description::{ImageDescription, mutate};

const CONFIG_FILE_PATH: &'static str = "Config.toml";

fn main() {
    let config = Config::from_config_file(CONFIG_FILE_PATH);
    println!("Config loaded: {:?}", config);

    let mut polygons: Vec<ColoredPolygon> = vec![];
    polygons.push(drawing::random_colored_triangle(config.width, config.height, config.alpha));

    let image_desc = ImageDescription {
        width: config.width,
        height: config.height,
        polygons: polygons
    };

    let image = drawing::draw_image(&image_desc);
    image.save("generated/before.png").unwrap();

    let new_image_desc = mutate(&image_desc, &config);

    let new_image = drawing::draw_image(&new_image_desc);
    new_image.save("generated/after.png").unwrap();
}
