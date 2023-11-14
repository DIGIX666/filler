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


pub fn get_previous_grid(std_input: String) -> Vec<String> {
    let grid_raw = std_input.split(":").collect::<Vec<&str>>();

    let gr1 = grid_raw[1].replace("Anfield ", "");
    let gr2 = gr1.replace(":", "");
    let gr3 = gr2.replace(" ", "");
    let gr4 = gr3.replace("\n", "");
    let mut grid_dirty: Vec<String> = Vec::new();

    grid_dirty = gr4
        .split("")
        .collect::<Vec<&str>>()
        .to_vec()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut grid_clean: Vec<String> = Vec::new();

    for elmt in grid_dirty.iter() {
        for (_k, e) in elmt.chars().enumerate() {
            if !e.is_ascii_alphanumeric() {
                grid_clean.push(e.to_string());
            }
        }
    }

    return grid_clean;
}

