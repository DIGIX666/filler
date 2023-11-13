use std::fs::File;
use std::io;

pub fn read_line() -> String {
    let mut input = String::new();

    // Lire une ligne depuis l'entrée standard (terminal)
    io::stdin()
        .read_line(&mut input)
        .expect("Erreur lors de la lecture de l'entrée");

    // Retirer le caractère de nouvelle ligne (\n) à la fin
    input.trim().to_string()
}