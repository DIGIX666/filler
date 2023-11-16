mod piece;
mod tools;

use piece::read_piece;
use std::{
    fs::File,
    io::{self, stderr, stdout, BufRead, BufReader, BufWriter, Write},
    process::Stdio,
};

use crate::tools::read_engine_output;

fn main() -> std::io::Result<()> {
    
    read_engine_output();

    println!("4 3");
    io::stdout().flush().expect("flush failed!");

    Ok(())
}
