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
