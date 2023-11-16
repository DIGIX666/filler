use std::fs::File;
use std::io::{self, BufRead, BufReader, Stdin, Write};

pub fn read_piece(std_input: String, piece_col: u32) -> Vec<Vec<String>> {
    let mut file = std::fs::File::create("piece.txt").unwrap();

    let mut engine_response_vec = std_input.split("\n").collect::<Vec<&str>>();

    writeln!(file, "piece vec: {:?}", engine_response_vec);

    let mut piece: Vec<Vec<String>> = Vec::new();
    // let mut piece_size:Vec<u32> = Vec::new();

    let mut good = false;
    let mut col: Vec<String> = Vec::new();

    let mut test_cut_piece = engine_response_vec[0].split("").collect::<Vec<&str>>();

    for i in 0..engine_response_vec.len() {
        for e in engine_response_vec[i].split("").collect::<Vec<&str>>() {
            // good = false;
            if !e.is_empty() {
                for (_i, char) in e.chars().enumerate() {
                    if (char.is_ascii_alphanumeric() || char.is_ascii_punctuation()) {
                        good = true;
                        col.push(char.to_string());
                    } else {
                        good = false;
                    }
                }
            }
        }
        piece.push(col.clone());
        col.clear();
        // if good && col.len() == piece_col as usize {
        //     piece.push(col.clone());
        // }
        // if !good {
        //     col.clear();
        // }
    }
    write!(file, "-->{:?}", piece);

    // println!("piece : {:?}", piece);
    return piece;
}

pub fn piece_coord(piece: Vec<Vec<String>>) -> (u32, u32) {
    let mut numLine: u32 = 0;
    let mut numCol: u32 = 0;

    for line in piece {
        for col in line {
            if col == "O" {
                numCol + 1;
            }
        }
        numLine + 1;
    }

    return (numCol, numLine);
}
