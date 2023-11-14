use std::fs::File;
use std::io::{self, BufRead};

pub fn read_line() -> String {
    let mut input = String::new();

    // Lire une ligne depuis l'entrée standard (terminal)
    let stdin = io::stdin();
    let reader = stdin.lock();
    // let mut input = String::new();

    // Itère sur chaque ligne
    for line_result in reader.lines() {
        match line_result {
            Ok(line) => {
                // Faites quelque chose avec la ligne lue
                println!("Ligne lue : {}", line);

                for (_k, e) in line.chars().enumerate() {
                        input.push(e);
                    
                }
            }
            Err(e) => {
                eprintln!("Erreur lors de la lecture de la ligne : {}", e);
                break;
            }
        }
    }
    // Retirer le caractère de nouvelle ligne (\n) à la fin
    return input;
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

