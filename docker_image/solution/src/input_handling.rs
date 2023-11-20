use std::io;
// use std::io::Write;
// use std::fs::OpenOptions;
use crate::place_piece::place_piece;

/****************************************************\
    Fonction pour traiter l'entrée et la sortie.
\****************************************************/
pub fn input_handling() {
    // let mut file = OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("output.txt")
    //     .expect("Failed to open output.txt");

    let (play, enemy) = get_player_characters();
    // writeln!(file, "play: {:?}", play);
    // writeln!(file, "enemy: {:?}", enemy);

    // Boucle sans fin pour traiter l'entrée continuellement.
    loop {
        // Lecture de la grille.
        let grid = read_grid();
        // writeln!(file, "grid: {:?}", grid);

        // Lecture de la pièce.
        let piece = read_piece();
        // writeln!(file, "piece: {:?}", piece);

        // Appelle la fonction `place_piece` pour trouver la position idéale de la pièce.
        let (piece_x, piece_y) = place_piece(&grid, &piece, &play, &enemy);

        // Affiche les coordonnées où la pièce doit être placée.
        println!("{} {}", piece_x, piece_y);
    }
}
//////////////////////// FIN //////////////////////////


/****************************************************\
    Fonction pour obtenir les caractères du joueur.
\****************************************************/
fn get_player_characters() -> (Vec<char>, Vec<char>) {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let player_number = input.chars().nth(10).unwrap();
    if player_number == '1' {
        (vec!['@', 'a'], vec!['$', 's'])
    } else {
        (vec!['$', 's'], vec!['@', 'a'])
    }
}
///////////////////////// FIN /////////////////////////


/****************************************************\
    Fonction pour lire la grille à partir de l'entrée.
\****************************************************/
fn read_grid() -> Vec<Vec<char>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let grid_details = input.split_whitespace().collect::<Vec<&str>>()[2];
    let grid_lines = grid_details[..grid_details.len() - 1]
        .parse::<i32>()
        .unwrap();

    // writeln!(file, "grid_details: {:?}", grid_details);
    // writeln!(file, "grid_line: {:?}", grid_lines);

    let mut grid = Vec::new();
    for i in 0..grid_lines + 1 {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if i < 1 {
            continue;
        } else {
            let row: Vec<char> = input[4..input.len() - 1].chars().collect();
            grid.push(row);
        }
    }
    grid
}
/////////////////////// FIN ////////////////////////////



/****************************************************\
    Fonction pour lire la pièce à partir de l'entrée.
\****************************************************/
fn read_piece() -> Vec<Vec<char>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let piece_details = input.split_whitespace().collect::<Vec<&str>>();
    let piece_lines = piece_details[2][..piece_details[2].len() - 1]
        .parse::<i32>()
        .unwrap();

    // writeln!(file, "piece_lines: {:?}", piece_lines);
    // writeln!(file, "piece_details: {:?}", piece_details);

    let mut piece = Vec::new();
    for _ in 0..piece_lines {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let row: Vec<char> = input[..input.len() - 1].chars().collect();
        piece.push(row);
    }
    piece
}
///////////////////////// FIN /////////////////////////////
