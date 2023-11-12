use std::io::{self, Read};

fn main() {
    // Lire tout le contenu de l'entrée standard (qui peut être redirigée depuis un fichier)
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Erreur de lecture de l'entrée standard");

    // Afficher le contenu lu depuis l'entrée standard (qui peut provenir d'un fichier)
    println!("Contenu du fichier :\n{}", buffer);
}
