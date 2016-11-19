use geo::boundingbox::BoundingBox;
use geo::contains::Contains;
use geo::{Point, LineString, Polygon};
use image::{ImageBuffer, Rgba, Pixel};
use rand::{thread_rng, Rng};

use image_description::ImageDescription;

pub type Color = Rgba<u8>;
pub type Image = ImageBuffer<Color, Vec<u8>>;

#[derive(Clone)]
pub struct ColoredPolygon {
    pub polygon: Polygon<f32>,
    pub color: Color,
}

pub fn random_color(alpha: u8) -> Color {
    let mut rng = thread_rng();
    let (r, g, b) = rng.gen::<(u8, u8, u8)>();
    Rgba([r, g, b, alpha])
}

fn random_point(width: u32, height: u32) -> Point<f32> {
    let mut rng = thread_rng();
    let x = rng.gen_range(0, width) as f32;
    let y = rng.gen_range(0, height) as f32;

    Point::new(x, y)
}

fn random_triangle(width: u32, height: u32) -> Polygon<f32> {
    let p1 = random_point(width, height);
    let p2 = random_point(width, height);
    let p3 = random_point(width, height);

    polygon_from_points(vec![p1, p2, p3, p1])
}

pub fn random_colored_triangle(width: u32, height: u32, alpha: u8) -> ColoredPolygon {
    ColoredPolygon {
        polygon: random_triangle(width, height),
        color: random_color(alpha),
    }
}

pub fn polygon_from_points(points: Vec<Point<f32>>) -> Polygon<f32> {
    let line_string = LineString(points);
    let exterior = vec![];
    Polygon::new(line_string.clone(), exterior.clone())
}

pub fn draw_image(description: &ImageDescription) -> Image {
    let mut image = ImageBuffer::from_pixel(description.width, description.height, Rgba([0u8, 0u8, 0u8, 255u8]));

    for polygon in &description.polygons {
        draw_polygon(&mut image, &polygon.polygon, polygon.color);
    }

    image
}

fn draw_polygon(image: &mut Image, polygon: &Polygon<f32>, color: Color) {
    let bounding_box = polygon.bbox().expect("Could not construct bounding box for polygon");
    let xmin = bounding_box.xmin as u32;
    let ymin = bounding_box.ymin as u32;
    let xmax = bounding_box.xmax as u32;
    let ymax = bounding_box.ymax as u32;

    for y in ymin..ymax {
        for x in xmin..xmax {
            let point = Point::new(x as f32, y as f32);
            if polygon.contains(&point) {
                let mut old_color = *image.get_pixel(x, y);
                old_color.blend(&color);
                image.put_pixel(x, y, old_color);
            }
        }
    }
}
