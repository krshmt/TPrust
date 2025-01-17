use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    let img_path = "images/IUT.jpg";
    let img = image::open(img_path).expect("Failed to open image");

    let (width, height) = img.dimensions();
    let mut img_buffer = ImageBuffer::new(width, height);
    let mut error_buffer = vec![vec![[0.0; 3]; width as usize]; height as usize];

    // Définition des couleurs de la palette
    let palette = [
        [0u8, 0, 0],       // Noir
        [255u8, 255, 255], // Blanc
        [255u8, 0, 0],     // Rouge
        [0u8, 0, 255],     // Bleu
        [0u8, 255, 0],     // Vert
    ];

    // Fonction pour trouver la couleur la plus proche dans la palette
    fn closest_palette_color(color: [f32; 3], palette: &[[u8; 3]]) -> [u8; 3] {
        palette
            .iter()
            .min_by_key(|&p| {
                ((color[0] - p[0] as f32).powi(2)
                    + (color[1] - p[1] as f32).powi(2)
                    + (color[2] - p[2] as f32).powi(2)) as u32
            })
            .cloned()
            .unwrap()
    }

    for y in 0..height {
        for x in 0..width {
            // Récupération du pixel avec ajout de l'erreur
            let pixel = img.get_pixel(x, y);
            let Rgba(data) = pixel;
            let original_color = [
                data[0] as f32 + error_buffer[y as usize][x as usize][0],
                data[1] as f32 + error_buffer[y as usize][x as usize][1],
                data[2] as f32 + error_buffer[y as usize][x as usize][2],
            ];

            // Trouver la couleur la plus proche dans la palette
            let new_color = closest_palette_color(original_color, &palette);

            // Placer le pixel transformé dans l'image de sortie
            img_buffer.put_pixel(x, y, Rgba([new_color[0], new_color[1], new_color[2], 255]));

            // Calcul de l'erreur
            let error = [
                original_color[0] - new_color[0] as f32,
                original_color[1] - new_color[1] as f32,
                original_color[2] - new_color[2] as f32,
            ];

            // Diffusion d'erreur selon Floyd-Steinberg
            if x + 1 < width {
                for c in 0..3 {
                    error_buffer[y as usize][(x + 1) as usize][c] += error[c] * 7.0 / 16.0;
                }
            }
            if y + 1 < height {
                if x > 0 {
                    for c in 0..3 {
                        error_buffer[(y + 1) as usize][(x - 1) as usize][c] += error[c] * 3.0 / 16.0;
                    }
                }
                for c in 0..3 {
                    error_buffer[(y + 1) as usize][x as usize][c] += error[c] * 5.0 / 16.0;
                }
                if x + 1 < width {
                    for c in 0..3 {
                        error_buffer[(y + 1) as usize][(x + 1) as usize][c] += error[c] * 1.0 / 16.0;
                    }
                }
            }
        }
    }

    // Sauvegarder l'image transformée
    img_buffer.save("images/diffusion-steinberg.png").expect("Failed to save image");
}
 