use std::io;

fn read_line() -> String {
    let mut input = String::new();

    // Lire une ligne depuis l'entrée standard (terminal)
    io::stdin().read_line(&mut input)
        .expect("Erreur lors de la lecture de l'entrée");

    // Retirer le caractère de nouvelle ligne (\n) à la fin
    input.trim().to_string()
}

fn main() {
    println!("Veuillez entrer quelque chose :");

    // Appeler la fonction pour lire l'entrée
    let entree = read_line();

    println!("Vous avez saisi : {}", entree);
}
