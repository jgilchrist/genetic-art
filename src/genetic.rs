use image::{Rgb, Pixel};

use drawing::{self, Image};
use image_description::ImageDescription;

pub fn fitness(input: &Image, image_desc: &ImageDescription) -> u32 {
    let rendered_image: Image = drawing::draw_image(image_desc);

    let (width, height) = input.dimensions();

    let mut total_error = 0;
    for x in 0..width {
        for y in 0..height {
            let p1 = input.get_pixel(x, y).to_rgb();
            let p2 = rendered_image.get_pixel(x, y).to_rgb();

            let perror = pixel_error(p1, p2);
            total_error += perror;
        }
    }

    total_error
}

fn pixel_error(p1: Rgb<u8>, p2: Rgb<u8>) -> u32 {
    let r = p1.data[0] as i32 - p2.data[0] as i32;
    let g = p1.data[1] as i32 - p2.data[1] as i32;
    let b = p1.data[2] as i32 - p2.data[2] as i32;

    (r*r + g*g + b*b) as u32
}
