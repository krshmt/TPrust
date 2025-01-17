use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    let img_path = "images/IUT.jpg";
    let img = image::open(img_path).expect("Failed to open image");

    let (width, height) = img.dimensions();
    let mut img_buffer = ImageBuffer::new(width, height);
    let mut error_buffer = vec![vec![0.0; width as usize]; height as usize];

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let Rgba(data) = pixel;
            let luminance = 0.299 * data[0] as f32 + 0.587 * data[1] as f32 + 0.114 * data[2] as f32;
            let corrected_luminance = luminance + error_buffer[y as usize][x as usize];

            let new_pixel = if corrected_luminance > 128.0 {
                Rgba([255u8, 255u8, 255u8, 255u8])
            } else {
                Rgba([0u8, 0u8, 0u8, 255u8])
            };

            img_buffer.put_pixel(x, y, new_pixel);

            let error = corrected_luminance - if corrected_luminance > 128.0 { 255.0 } else { 0.0 };

            if x + 1 < width {
                error_buffer[y as usize][(x + 1) as usize] += error * 0.5;
            }
            if y + 1 < height {
                error_buffer[(y + 1) as usize][x as usize] += error * 0.5;
            }
        }
    }

    img_buffer.save("images/image_diffusion.png").expect("Failed to save image");
}
