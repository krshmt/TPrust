use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    // Chemin de l'image
    let img_path = "images/IUT.jpg";

    let img = image::open(img_path).expect("echec de l'ouverture de l'image");

    let (width, height) = img.dimensions();

    // Définir les 8 couleurs
    let palette: [Rgba<u8>; 8] = [
        Rgba([0, 0, 0, 255]),       // Noir
        Rgba([255, 255, 255, 255]), // Blanc
        Rgba([255, 0, 0, 255]),     // Rouge
        Rgba([0, 255, 0, 255]),     // Vert
        Rgba([0, 0, 255, 255]),     // Bleu
        Rgba([255, 255, 0, 255]),   // Jaune
        Rgba([255, 0, 255, 255]),   // Magenta
        Rgba([0, 255, 255, 255]),   // Cyan
    ];

    let mut img_buffer = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let Rgba(data) = pixel;

        let closest_color = if palette.is_empty() {
            Rgba([0, 0, 0, 255])
        } else {
            *palette.iter().min_by_key(|&&color| {
                let dr = color[0] as i32 - data[0] as i32;
                let dg = color[1] as i32 - data[1] as i32;
                let db = color[2] as i32 - data[2] as i32;
                dr * dr + dg * dg + db * db
            }).unwrap()
        };

        img_buffer.put_pixel(x, y, *closest_color);
    }

    let output_path = "images/IUT_palette.png";
    img_buffer.save(output_path).expect("echec de l'enregistrement de l'image");
}
