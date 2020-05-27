use config::Config;
use image::{ImageBuffer, Rgb, Pixel};
use rand::{thread_rng, Rng};
use std::cmp;

pub type Color = Rgb<u8>;
pub type Image = ImageBuffer<Color, Vec<u8>>;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64
}

#[derive(Clone, Debug)]
pub struct Circle {
    pub location: Point,
    pub radius: f64,
}

#[derive(Clone, Debug)]
pub struct ColoredCircle {
    pub circle: Circle,
    pub color: Color,
}

#[derive(Clone, Debug)]
pub struct ImageDescription {
    pub width: u64,
    pub height: u64,
    pub items: Vec<ColoredCircle>
}

pub fn mutate(old_image: &ImageDescription, config: &Config) -> ImageDescription {
    let mut image = old_image.clone();

    add_circle(&mut image, config);
    remove_circle(&mut image, config);
    move_circle(&mut image, config);
    alter_circle_color(&mut image, config);

    image
}

#[inline]
fn should_mutate(chance: f64) -> bool {
    let r: f64 = rand::thread_rng().gen();
    return r < chance;
}

pub fn add_circle(image: &mut ImageDescription, config: &Config) {
    if !should_mutate(config.add_chance) { return; }

    image.items.push(
        random_colored_circle(image.width, image.height, config.max_radius)
    );
}

pub fn remove_circle(image: &mut ImageDescription, config: &Config) {
    if !should_mutate(config.remove_chance) { return; }
    if image.items.is_empty() { return; }

    let random_index = rand::thread_rng().gen_range(0, image.items.len());
    image.items.remove(random_index);
}

pub fn move_circle(image: &mut ImageDescription, config: &Config) {
    if !should_mutate(config.move_chance) { return; }
    if image.items.is_empty() { return; }

    let x_amount = rand::thread_rng().gen_range(-config.max_move_amount, config.max_move_amount) as i64;
    let y_amount = rand::thread_rng().gen_range(-config.max_move_amount, config.max_move_amount) as i64;

    let random_index = rand::thread_rng().gen_range(0, image.items.len());
    let random_circle = image.items.remove(random_index);

    let new_location = Point { x: random_circle.circle.location.x as i64 + x_amount, y: random_circle.circle.location.y as i64 + y_amount };

    let new_colored_circle = ColoredCircle {
        circle: Circle { location: new_location, radius: random_circle.circle.radius },
        color: random_circle.color,
    };

    image.items.push(new_colored_circle);
}

pub fn alter_circle_color(image: &mut ImageDescription, config: &Config) {
    if !should_mutate(config.alter_color_chance) { return; }
    if image.items.is_empty() { return; }

    let random_index = rand::thread_rng().gen_range(0, image.items.len());
    let random_circle = image.items.remove(random_index);

    let new_color = random_circle.color.map(|v| {
        let old_value = v as i64;
        let mut new_value = old_value + rand::thread_rng().gen_range(-30, 30);
        new_value = cmp::max(new_value, 0);
        new_value = cmp::min(new_value, 255);
        new_value as u8
    });

    let new_colored_polygon = ColoredCircle {
        circle: random_circle.circle,
        color: new_color,
    };

    image.items.push(new_colored_polygon);
}

pub fn random_color() -> Color {
    let mut rng = thread_rng();
    let (r, g, b) = rng.gen::<(u8, u8, u8)>();
    Rgb([r, g, b])
}

fn random_circle(width: u64, height: u64, radius_limit: u64) -> Circle {
    let mut rng = thread_rng();
    let x = rng.gen_range(0, width) as i64;
    let y = rng.gen_range(0, height) as i64;

    let location = Point { x, y };

    let radius = rng.gen_range(0, radius_limit) as f64;

    Circle { location, radius }
}

pub fn random_colored_circle(width: u64, height: u64, radius_limit: u64) -> ColoredCircle {
    ColoredCircle {
        circle: random_circle(width, height, radius_limit),
        color: random_color(),
    }
}
