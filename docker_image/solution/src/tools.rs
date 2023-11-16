use std::fs::File;
use std::io::{self, BufRead, BufReader,Write, Stdin};

use crate::piece;

pub fn read_engine_output() /*-> (String,String,String)*/ {
    let mut input = String::new();
    let mut player = String::new();
    let mut engine_output = String::new();
    let mut gz = String::new();
    let mut length_engine_grid: u32 = 0;

    let mut reader = BufReader::new(io::stdin());


    let mut file = File::create("output.txt").unwrap();

    reader.read_line(&mut player).unwrap();
    writeln!(file, "{}", player);

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
    // writeln!(file, "piece_line: {}", piece_line);


    let mut piece = String::new();

    for _i in 0..piece_line.trim().parse::<u32>().unwrap() {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        // writeln!(file, "'{}'", line);
        // writeln!(file, "line: {}", line.trim());
        piece.push_str(&line.trim());
    }

    // let mut piece = String::new();
    // reader.read_line(&mut piece).unwrap();
    writeln!(file, "{}", piece);

    let mut piece_column = piece_col.trim().parse::<u32>().unwrap();

    let mut piece = piece::read_piece(piece,piece_column);
    writeln!(file, "piece: {:?}", piece);
    
    let col = piece::piece_coord(piece.clone()).0;
    let line = piece::piece_coord(piece.clone()).1;

    // writeln!(file, "piece column: {}", col);
    // writeln!(file, "piece line: {}", line);


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

// pub fn get_player(){
//     let mut player = String::new();
//     let mut reader = BufReader::new(io::stdin());
//     let mut file = File::create("output.txt").unwrap();
//     let _ = reader.read_line(&mut player);
//     writeln!(file, "player: {}", player).unwrap();

//     // println!("player: {}", player);
// }

pub fn get_grid(){
}

