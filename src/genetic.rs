use image::{Rgb, Pixel};

use drawing::{self, Image};
use image_description::ImageDescription;

pub fn fitness(input: &Image, image_desc: &ImageDescription) -> u32 {
    let rendered_image: Image = drawing::draw_image(image_desc);

    let zipped_pixels = input.pixels().zip(rendered_image.pixels());

    let mut total_error = 0;
    for (p1rgba, p2rgba) in zipped_pixels {
        let p1rgb = p1rgba.to_rgb();
        let p2rgb = p2rgba.to_rgb();

        let perror = pixel_error(p1rgb, p2rgb);
        total_error += perror;
    }

    total_error
}

fn pixel_error(p1: Rgb<u8>, p2: Rgb<u8>) -> u32 {
    let r = p1.data[0] as u32 - p2.data[0] as u32;
    let g = p1.data[1] as u32 - p2.data[1] as u32;
    let b = p1.data[2] as u32 - p2.data[2] as u32;

    r*r + g*g + b*b
}
