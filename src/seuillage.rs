use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    let img_path = "images/IUT.jpg";
    let img = image::open(img_path).expect("Failed to open image");

    let (width, height) = img.dimensions();
    let mut img_buffer = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        if (x + y) % 2 == 0 {
            img_buffer.put_pixel(x, y, Rgba([255, 255, 255, 255]));
        } else {
            img_buffer.put_pixel(x, y, pixel);
        }
    }

    img_buffer.save("images/image_modifiee.png").expect("Failed to save image");
}