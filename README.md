# Kris TOURE et Rémi BOULAY
## TP Rust - 2025

1. Objectif du projet

    Réaliser quelques manipulations d’images à l’aide de la bibliothèque rust image

2. Exécuter nos codes

    #### Pour faire "Passage en monochrome par seuillage" : 

    ```bash
    cd src/
    cargo build
    cargo run -- bin seuillage
    ```

    Après l'éxecution du code, l'image doit se trouver dans le fichier src/images/IUT_seuillage.png 

    <br>

    #### Passer l'image sur une palette de 8 couleurs:

    ```bash
    cd src/
    cargo build
    cargo run --bin palette
    ```
   
    Après l'éxecution du code, l'image se trouve dans le fichier src/images/IUT_palette.png 

    <br>

    #### Pour passer l'image sur la matrice de Bayer
    ```bash
    cd src/
    cargo build
    cargo run --bin matrice_de_Bayer
    ```
   
   Après l'éxecution du code, l'image se trouve dans le fichier src/images/IUT_bayer.png 

    <br>

    #### Pour faire "Tramage aléatoire (dithering)" : 

    ```bash
    cd src/
    cargo build
    cargo run --bin tramage
    ```

    Après l'éxecution du code, l'image doit se trouver dans le fichier src/images/IUT_tramage.png 

    <br>

    #### Pour faire "Diffusion d'erreurs" :

    ```bash
    cd src/
    cargo build
    cargo run --bin diffusion
    ```

    Après l'éxecution du code, l'image doit se trouver dans le fichier src/images/image_diffusion.png

    <br>

    #### Pour faire "Diffusion d'erreurs" avec la palette de couleur :

    ```bash
    cd src/
    cargo build
    cargo run --bin diffusion-palette
    ```

    Après l'éxecution du code, l'image doit se trouver dans le fichier src/images/diffusion-palette.png

    <br>

    #### Pour faire "Diffusion d'erreurs" avec la matrice Floyd-Steinberg

    ```bash
    cd src/
    cargo build
    cargo run --bin diffusion-steinberg
    ```

    Après l'éxecution du code, l'image doit se trouver dans le fichier src/images/diffusion-steinberg.png

3. Réponses aux questions

### Question 2:

Le type DynamicImage correspond a une image dans la mémoire.
Pour obtenir une image en mode rbg8 à partir de ce DynamicImage on peut utiliser la méthode to_rgb8() qui renvoie une image au format RGB avec 8 bits par canal.

### Question 3:

Le canal alpha est supprimé. L'image contiendra uniquement trois canaux (Rouge, Vert, Bleu).

### Question 5:

L'image est reconnaissable

### Question 9:

Dans le programme, la méthode pour calculer la distance entre les deux couleurs est la distance euclidienne au carré. La formule utilisé est : 

distance : $` (R1 - R2)^2 + (G1 - G2)^2 + (B1 - B2)^2 `$

### Question 11:

Pour gérer l'exception de la palette vide, nous avons choisi de mettre une couleur par défaut pour remplacer les pixels. La valeur choisi est noire. On pouvait avoir d'autres solutions comme :

- Remplacer les pixels par du blanc
- Ne pas modifier l'image/mettre une erreur en renvoyant "La palette est vide"

```rust
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
```

### Question 14:

La matrice de Bayer représentée sous la forme :
```rust
Vec<Vec<u32>
```

Pour créer la matrice de Bayer d'ordre arbitraire, nous avons crée la fonction récursive "generate_bayer_matrix"

```rust
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
```

