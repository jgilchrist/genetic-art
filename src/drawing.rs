use super::Image;

use geo::{Point, LineString, Polygon};
use geo::contains::Contains;
use geo::boundingbox::BoundingBox;
use image::{Rgba, Pixel};
use rand::{thread_rng, Rng};

pub fn random_color(alpha: u8) -> Rgba<u8> {
    let mut rng = thread_rng();
    let (r, g, b) = rng.gen::<(u8, u8, u8)>();
    Rgba([r, g, b, alpha])
}

pub fn random_point(width: u32, height: u32) -> Point<f32> {
    let mut rng = thread_rng();
    let x = rng.gen_range(0, width) as f32;
    let y = rng.gen_range(0, height) as f32;

    Point::new(x, y)
}

pub fn random_triangle(width: u32, height: u32) -> Polygon<f32> {
    let p1 = random_point(width, height);
    let p2 = random_point(width, height);
    let p3 = random_point(width, height);

    polygon_from_points(vec![p1, p2, p3, p1])
}

pub fn polygon_from_points(points: Vec<Point<f32>>) -> Polygon<f32> {
    let line_string = LineString(points);
    let exterior = vec![];
    Polygon::new(line_string.clone(), exterior.clone())
}

pub fn draw_polygon(old_image: &Image, polygon: &Polygon<f32>, color: Rgba<u8>) -> Image {
    let mut image = old_image.clone();

    let bounding_box = polygon.bbox().expect("Could not construct bounding box for polygon");
    let xmin = bounding_box.xmin as u32;
    let ymin = bounding_box.ymin as u32;
    let xmax = bounding_box.xmax as u32;
    let ymax = bounding_box.ymax as u32;

    for y in ymin..ymax {
        for x in xmin..xmax {
            let point = Point::new(x as f32, y as f32);
            if polygon.contains(&point) {
                let mut old_color = image.get_pixel(x, y).clone();
                old_color.blend(&color);
                image.put_pixel(x, y, old_color);
            }
        }
    }

    image
}
