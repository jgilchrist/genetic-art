use super::Image;
use shapes::{Point, Triangle};

use image::{ImageBuffer, Rgba};
use rand::{thread_rng, Rng};

pub fn random_point(max_size: u32) -> Point {
    let mut rng = thread_rng();
    let x = rng.gen_range(0, max_size);
    let y = rng.gen_range(0, max_size);

    Point::new(x, y)
}

pub fn random_triangle(max_size: u32) -> Triangle {
    let mut rng = thread_rng();
    let p1 = random_point(max_size);
    let p2 = random_point(max_size);
    let p3 = random_point(max_size);

    Triangle::new(p1, p2, p3)
}

pub fn draw_triangle(old_image: &Image, triangle: &Triangle) -> Image {
    let mut image = old_image.clone();

    image
}
