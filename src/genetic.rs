use image::{Rgb, Pixel};
use drawing::{Image, ImageDescription};
use rendering;

pub fn fitness(input: &Image, image_desc: &ImageDescription) -> u64 {
    let rendered_image: Image = rendering::draw_image(image_desc);

    let (width, height) = input.dimensions();

    let mut total_error: u64 = 0;
    for x in 0..width {
        for y in 0..height {
            let p1 = input.get_pixel(x, y).to_rgb();
            let p2 = rendered_image.get_pixel(x, y).to_rgb();

            let perror = pixel_error(p1, p2);
            total_error = total_error.saturating_add(perror);
        }
    }

    total_error
}

fn pixel_error(p1: Rgb<u8>, p2: Rgb<u8>) -> u64 {
    let Rgb([r1, g1, b1]) = p1;
    let Rgb([r2, g2, b2]) = p2;
    ((r1 as i64 - r2 as i64).pow(2) + (g1 as i64 - g2 as i64).pow(2) + (b1 as i64 - b2 as i64).pow(2)) as u64
}
