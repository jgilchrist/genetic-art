use super::Image;
use shapes::{Point, Triangle};
use math;

use image::Rgba;
use rand::{thread_rng, Rng};

pub fn random_point(width: u32, height: u32) -> Point {
    let mut rng = thread_rng();
    let x = rng.gen_range(0, width) as i32;
    let y = rng.gen_range(0, height) as i32;

    Point::new(x, y)
}

pub fn random_triangle(width: u32, height: u32) -> Triangle {
    let p1 = random_point(width, height);
    let p2 = random_point(width, height);
    let p3 = random_point(width, height);

    Triangle::new(p1, p2, p3)
}

pub fn draw_triangle(old_image: &Image, triangle: &Triangle) -> Image {
    let mut image = old_image.clone();

    let (top_left, bottom_right) = get_bounding_box_for_triangle(&triangle);

    for y in top_left.y..bottom_right.y {
        for x in top_left.x..bottom_right.x {
            let point = Point::new(x, y);
            if point_is_in_triangle(&point, &triangle) {
                image.put_pixel(x as u32, y as u32, Rgba([255u8, 255u8, 255u8, 255u8]));
            }
        }
    }

    image
}

fn get_bounding_box_for_triangle(triangle: &Triangle) -> (Point, Point) {
    let min_x = math::min3(triangle.p0.x, triangle.p1.x, triangle.p2.x);
    let min_y = math::min3(triangle.p0.y, triangle.p1.y, triangle.p2.y);
    let max_x = math::max3(triangle.p0.x, triangle.p1.x, triangle.p2.x);
    let max_y = math::max3(triangle.p0.y, triangle.p1.y, triangle.p2.y);

    (Point::new(min_x, min_y), Point::new(max_x, max_y))
}

fn point_is_in_triangle(point: &Point, triangle: &Triangle) -> bool {
    let p0 = &triangle.p0;
    let p1 = &triangle.p1;
    let p2 = &triangle.p2;
    let pt = &point;

    let a = -p1.y * p2.x + p0.y * (-p1.x + p2.x) + p0.x * (p1.y - p2.y) + p1.x * p2.y;
    let sign = if a < 0 {-1} else {1};
    let s = (p0.y * p2.x - p0.x * p2.y + (p2.y - p0.y) * pt.x + (p0.x - p2.x) * pt.y) * sign;
    let t = (p0.x * p1.y - p0.y * p1.x + (p0.y - p1.y) * pt.x + (p1.x - p0.x) * pt.y) * sign;

    s > 0 && t > 0 && (s + t) < a * sign
}
