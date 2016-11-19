use std::cmp;

use rand::{self, Rng};
use geo::Point;
use image::Pixel;

use config::Config;
use drawing::{self, random_colored_triangle, ColoredPolygon};

#[derive(Clone)]
pub struct ImageDescription {
    pub width: u32,
    pub height: u32,
    pub polygons: Vec<ColoredPolygon>
}

pub fn mutate(old_image: &ImageDescription, config: &Config) -> ImageDescription {
    let mut image = old_image.clone();

    add_polygon(&mut image, config);
    remove_polygon(&mut image, config);
    move_polygon(&mut image, config);
    alter_polygon_color(&mut image, config);

    image
}

#[inline]
fn should_mutate(chance: f32) -> bool {
    rand::thread_rng().next_f32() < chance
}

pub fn add_polygon(image: &mut ImageDescription, config: &Config) {
    if !should_mutate(config.add_polygon_chance) { return; }

    let new_polygon = random_colored_triangle(config.width, config.height, config.alpha);
    image.polygons.push(new_polygon);
}

pub fn remove_polygon(image: &mut ImageDescription, config: &Config) {
    if !should_mutate(config.remove_polygon_chance) { return; }
    if image.polygons.is_empty() { return; }

    let random_index = rand::thread_rng().gen_range(0, image.polygons.len());
    image.polygons.remove(random_index);
}

pub fn move_polygon(image: &mut ImageDescription, config: &Config) {
    if !should_mutate(config.move_polygon_chance) { return; }
    if image.polygons.is_empty() { return; }

    let x_amount = rand::thread_rng().gen_range(-config.max_move_amount, config.max_move_amount);
    let y_amount = rand::thread_rng().gen_range(-config.max_move_amount, config.max_move_amount);
    let point_to_add = Point::new(x_amount, y_amount);

    let random_index = rand::thread_rng().gen_range(0, image.polygons.len());
    let random_polygon = image.polygons.remove(random_index);

    let new_exterior = random_polygon
        .polygon
        .exterior.0
        .iter()
        .map(|e| *e + point_to_add)
        .collect::<Vec<_>>();

    let new_polygon = drawing::polygon_from_points(new_exterior);

    let new_colored_polygon = ColoredPolygon {
        polygon: new_polygon,
        color: random_polygon.color,
    };

    image.polygons.push(new_colored_polygon);
}

pub fn alter_polygon_color(image: &mut ImageDescription, config: &Config) {
    if !should_mutate(config.alter_polygon_color_chance) { return; }
    if image.polygons.is_empty() { return; }

    let random_index = rand::thread_rng().gen_range(0, image.polygons.len());
    let random_polygon = image.polygons.remove(random_index);

    let new_color = random_polygon.color.map(|v| {
        let old_value = v as i32;
        let mut new_value = old_value + rand::thread_rng().gen_range(-30, 30);
        new_value = cmp::max(new_value, 0);
        new_value = cmp::min(new_value, 255);
        new_value as u8
    });

    let new_polygon = drawing::polygon_from_points(random_polygon.polygon.exterior.0);

    let new_colored_polygon = ColoredPolygon {
        polygon: new_polygon,
        color: new_color,
    };

    image.polygons.push(new_colored_polygon);
}
