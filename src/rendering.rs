use image::{ImageBuffer, Rgb};
use drawing::{ColoredCircle, Image, ImageDescription, Point};

pub fn draw_image(description: &ImageDescription) -> Image {
    let mut image = ImageBuffer::from_pixel(description.width as u32, description.height as u32, Rgb([0u8, 0u8, 0u8]));

    for circle in &description.items {
        draw_circle(&mut image, circle);
    }

    image
}

fn draw_circle(image: &mut Image, colored_circle: &ColoredCircle) {
    let radius = colored_circle.circle.radius as i64;

    for y in -radius..radius {
        for x in -radius..radius {
            if x.pow(2) + y.pow(2) <= radius.pow(2) {
                let draw_x = colored_circle.circle.location.x as i64 + x;
                let draw_y = colored_circle.circle.location.y as i64 + y;
                if point_in_image(image, Point { x: draw_x, y: draw_y }) {
                    image.put_pixel(draw_x as u32, draw_y as u32, colored_circle.color);
                }
            }
        }
    }

    // for angle_step in 0..3600 {
    //     let angle = angle_step as f64 * 0.1;
    //     let xdiff = colored_circle.circle.radius * (angle * std::f64::consts::PI / 180.0).cos();
    //     let ydiff = colored_circle.circle.radius * (angle * std::f64::consts::PI / 180.0).sin();
    //     let cx = colored_circle.circle.location.x as i64;
    //     let cy = colored_circle.circle.location.y as i64;

    //     let x = cx + xdiff as i64;
    //     let y = cy + ydiff as i64;

    //     let point = Point {
    //         x: if x > 0 { x as u64 } else { 0 },
    //         y: if y > 0 { y as u64 } else { 0 },
    //     };

    //     if point_in_image(image, point) {
    //     }
    // }
}

fn point_in_image(image: &Image, point: Point) -> bool {
    let (width, height) = image.dimensions();
    point.x >= 0 && point.y >= 0 && point.x < width as i64 && point.y < height as i64
}
