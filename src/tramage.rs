use image::{GenericImageView, ImageBuffer, Rgba};
use rand::Rng;

fn main() {
    let img_path = "images/IUT.jpg";
    let img = image::open(img_path).expect("Failed to open image");

    let (width, height) = img.dimensions();
    let mut img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let mut rng = rand::thread_rng();

    for (x, y, pixel) in img.pixels() {
        let Rgba(data) = pixel;
        let luminance = 0.299 * data[0] as f32 + 0.587 * data[1] as f32 + 0.114 * data[2] as f32;
        let seuil: f32 = rng.gen(); // Seuil aléatoire
        if luminance / 255.0 > seuil {
            img_buffer.put_pixel(x, y, Rgba([255, 255, 255, 255])); 
        } else {
            img_buffer.put_pixel(x, y, Rgba([0, 0, 0, 255])); 
        }
    }

    img_buffer
        .save("images/image_tramage.png")
        .expect("Failed to save image");
}
