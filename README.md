# Kris TOURE et Rémi BOULAY
## TP Rust - 2025

1. Objectif du projet

    Réaliser quelques manipulations d’images à l’aide de la bibliothèque rust image

2. Exécuter nos codes

    Pour faire "Passage en monochrome par seuillage" : 

    ```bash
    cd src/
    cargo build
    cargo run
    ```

    Après l'éxecution du code, l'image doit se trouver dans le fichier src/images/IUT_seuillage.png 



    Passer l'image sur une palette de 8 couleurs:

   ```cargo run --bin palette```
   
    Après l'éxecution du code, l'image se trouve dans le fichier src/images/IUT_palette.png 



   Pour passer l'image sur la matrice de Bayer

   ```cargo run --bin matrice_de_Bayer```
   
   Après l'éxecution du code, l'image se trouve dans le fichier src/images/IUT_bayer.png 



    Pour faire "Tramage aléatoire (dithering)" : 

    ```bash
    cd src/
    cargo build
    cargo run --bin tramage
    ```

    Après l'éxecution du code, l'image doit se trouver dans le fichier src/images/IUT_tramage.png 
