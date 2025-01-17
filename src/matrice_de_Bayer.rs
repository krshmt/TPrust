use image::{GenericImageView, ImageBuffer, Rgba};


fn generate_bayer_matrix(order: u32) -> Vec<Vec<u32>> {
    if order == 0 {
        return vec![vec![0]];
    }

    let prev = generate_bayer_matrix(order - 1);
    let size = prev.len();
    let mut matrix = vec![vec![0; size * 2]; size * 2];

    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = 4 * prev[i][j];
            matrix[i][j + size] = 4 * prev[i][j] + 2;
            matrix[i + size][j] = 4 * prev[i][j] + 3;
            matrix[i + size][j + size] = 4 * prev[i][j] + 1;
        }
    }

    matrix
}

fn main() {
    let img_path = "images/IUT.jpg";

    let img = image::open(img_path).expect("echec de l'ouverture de l'image");

    let (width, height) = img.dimensions();

    let bayer_matrix = generate_bayer_matrix(4);
    let matrix_size = bayer_matrix.len();

    let max_value = (matrix_size * matrix_size) as u32;
    let bayer_matrix: Vec<Vec<u8>> = bayer_matrix
        .iter()
        .map(|row| row.iter().map(|&val| ((val * 255) / max_value) as u8).collect())
        .collect();

    let mut img_buffer = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let Rgba(data) = pixel;

        let luminance = (0.299 * data[0] as f32 + 0.587 * data[1] as f32 + 0.114 * data[2] as f32) as u8;

        let threshold = bayer_matrix[y as usize % matrix_size][x as usize % matrix_size];

        if luminance > threshold {
            img_buffer.put_pixel(x, y, Rgba([255, 255, 255, 255])); // Blanc
        } else {
            img_buffer.put_pixel(x, y, Rgba([0, 0, 0, 255])); // Noir
        }
    }

    // Sauvegarder l'image
    let output_path = "images/IUT_bayer.png";
    img_buffer.save(output_path).expect("echec de l'enregistrement de l'image");
    println!("Image sauvegarder dans {}", output_path);
}
