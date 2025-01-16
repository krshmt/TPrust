use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    let img_path = "images/IUT.jpg";
    let img = image::open(img_path).expect("Failed to open image");

    let (width, height) = img.dimensions();
    let mut img_buffer = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let Rgba(data) = pixel;
        let luminance = 0.299 * data[0] as f32 + 0.587 * data[1] as f32 + 0.114 * data[2] as f32;
        if luminance > 128.0 {
            img_buffer.put_pixel(x, y, Rgba([255u8, 255u8, 255u8, 255u8]));
        } else {
            img_buffer.put_pixel(x, y, Rgba([0u8, 0u8, 0u8, 255u8]));
        }
    }

    img_buffer.save("images/IUT_seuillage.png").expect("Failed to save image");
}
