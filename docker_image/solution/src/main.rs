mod piece;
mod tools;

use piece::read_piece;
use std::{
    fs::File,
    io::{self, stderr, stdout, BufRead, BufReader, BufWriter, Write},
    process::Stdio,
};

use crate::tools::{get_previous_grid_dirty, read_engine_output};


fn main() -> io::Result<()> {

    // let file = File::create("output.txt")?;
    
    read_engine_output();
    // writeln!(file, "player--> {}", output)?;

    println!("7 5");

    io::stdout().flush().unwrap();

    Ok(())
}
