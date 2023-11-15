mod piece;
mod tools;

use piece::read_piece;
// use tools::read_line;
use std::{
    fs::File,
    io::{self, stderr, stdout, BufRead, BufReader, BufWriter, Write},
    process::Stdio,
};

use crate::tools::read_engine_output;

fn main() -> std::io::Result<()> {

    let mut file = File::create("output.txt").unwrap();
    let mut reader = BufReader::new(io::stdin());

    // writeln!(file, "== engine_output: {}", engine_output)?;
    // let grid = tools::get_previous_grid_dirty(read_engine_output());
    // reader.read_line(&mut grid).unwrap();
    // writeln!(file, "== grid: {}", grid)?;

    println!("4 3");
    io::stdout().flush().unwrap();

    Ok(())
}
