use std::fs::File;
use std::io::{self, BufRead, BufReader,Write, Stdin};

use crate::piece;

pub fn read_engine_output() /*-> (String,String,String)*/ {
    let mut input = String::new();
    let mut player = String::new();
    let mut pstring = Vec::new(); // player string
    let mut estring = Vec::new(); // ennemy string

    let mut engine_output = String::new();
    let mut gz = String::new();
    let mut length_engine_grid: u32 = 0;


    let mut reader = BufReader::new(io::stdin());


    let mut file = File::create("output.txt").unwrap();

   
    reader.read_line(&mut player).unwrap();
    writeln!(file, "{}", player);

    if player.contains("p1") {
        pstring = vec!['@', 'a'];
        estring = vec!['$', 's'];
    } else {
        pstring = vec!['$', 's'];
        estring = vec!['@', 'a'];
    }

    reader.read_line(&mut gz).unwrap();
    writeln!(file, "{}", gz);

    let mut grid_size = String::new();
    gz = gz.replace(":", "");
    // gz = gz.replace(" ", "");
    gz = gz.replace("\n", "");
    grid_size = gz.trim().to_string().split(" ").collect::<Vec<&str>>()[2].to_string();
    writeln!(file, "== grid_size: {}", grid_size);

    length_engine_grid = grid_size.trim().parse::<u32>().unwrap();
    writeln!(file, "== length_engine_grid: {}", length_engine_grid);

    for _i in 0..length_engine_grid {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        // writeln!(file, "'{}'", line);
        // writeln!(file, "line: {}", line.trim());
        engine_output.push_str(&line);
    }
    reader.read_line(&mut engine_output).unwrap();
    // return (player,grid_size,engine_output);

    let grid_clean = get_previous_grid_dirty(engine_output);

    writeln!(file, "grid_clean: {}", grid_clean);

    let mut piece_size = String::new();
    reader.read_line(&mut piece_size).unwrap();
    writeln!(file, "piece size: {}", piece_size);

    piece_size = piece_size.replace("Piece", "");
    piece_size = piece_size.replace(":", "");
    let mut piece_line = piece_size.trim().to_string().split(" ").collect::<Vec<&str>>()[1].to_string();
    let mut piece_col: String = piece_size.trim().to_string().split(" ").collect::<Vec<&str>>()[0].to_string();


    let mut piece = String::new();

    for _i in 0..piece_line.trim().parse::<u32>().unwrap() {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        // Ajouter la ligne à la pièce si elle n'est pas vide
        if !line.trim().is_empty() {
            piece.push_str(&line);
        }
    }
    
    // Supprimer le dernier caractère si c'est une nouvelle ligne
    if piece.ends_with('\n') {
        piece.pop();
    }
    
    writeln!(file, "{}", piece);

    let mut piece_column = piece_col.trim().parse::<u32>().unwrap();

    let mut piece = piece::read_piece(piece,piece_column);
    writeln!(file, "piece: {:?}", piece);
    
    let col = piece::piece_coord(piece.clone()).0;
    let line = piece::piece_coord(piece.clone()).1;
}


pub fn get_previous_grid_dirty(std_input: String) -> String {
   
    let mut grid_dirty: String = String::new();
    

    for e in std_input.chars() {
        
            if !e.is_ascii_alphanumeric() {
                grid_dirty.push(e);
            }
    }
    return grid_dirty;
}


pub fn place_piece(grid: &Vec<Vec<char>>, piece: &Vec<Vec<String>>, pstring: &String, estring: &String, player: String) -> (u32, u32) {
    let mut piece_x: u32 = 0;
    let mut file = File::create("output.txt").unwrap();
    let mut piece_y: u32 = 0;
    let mut sol = (0, 0);
    let (mut xmin, mut xmax, mut ymin, mut ymax) = (0, 0, 0, 0);

    let pstring_chars: Vec<char> = pstring.chars().collect();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == pstring_chars[0] || grid[i][j] == pstring_chars[1] { 
                if i < xmin {
                    xmin = i;
                }
                if i > xmax {
                    xmax = i;
                }
                if j < ymin {
                    ymin = j;
                }
                if j > ymax {
                    ymax = j;
                }
            }
        }
    }

    // Utilisez la variable `distance` ou supprimez-la si elle n'est pas nécessaire
    // let mut distance = ((grid.len() as f32).powf(2.) + (grid[0].len() as f32).powf(2.)).sqrt();

    writeln!(file, "sol3: {:?}", sol);
    sol

}