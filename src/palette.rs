use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    // Chemin de l'image source
    let img_path = "images/IUT.jpg";

    // Charger l'image
    let img = image::open(img_path).expect("Failed to open image");

    // Récupérer les dimensions de l'image
    let (width, height) = img.dimensions();

    // Définir les 8 couleurs de la palette
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

    // Créer un nouveau tampon d'image pour le résultat
    let mut img_buffer = ImageBuffer::new(width, height);

    // Parcourir chaque pixel de l'image source
    for (x, y, pixel) in img.pixels() {
        let Rgba(data) = pixel;

        // Trouver la couleur de la palette la plus proche
        let closest_color = palette.iter().min_by_key(|&&color| {
            // Calculer la distance euclidienne dans l'espace RGB
            let dr = color[0] as i32 - data[0] as i32;
            let dg = color[1] as i32 - data[1] as i32;
            let db = color[2] as i32 - data[2] as i32;
            dr * dr + dg * dg + db * db
        }).unwrap();

        // Mettre à jour le pixel dans le tampon
        img_buffer.put_pixel(x, y, *closest_color);
    }

    // Sauvegarder l'image résultante
    let output_path = "images/IUT_palette.png";
    img_buffer.save(output_path).expect("Failed to save image");
    println!("Image saved to {}", output_path);
}
