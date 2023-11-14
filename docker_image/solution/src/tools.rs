use std::fs::File;
use std::io::{self, BufRead, BufReader,Write};

pub fn read_engine_output() -> String {
    let mut input = String::new();
    let mut player = String::new();
    let mut engine_output = String::new();
    let mut gz = String::new();
    let mut length_engine_grid: u32 = 0;

    let mut reader = BufReader::new(io::stdin());

    // Lire une ligne depuis l'entrée standard (terminal)

    // let mut file = File::create("output.txt").unwrap();

    reader.read_line(&mut player).unwrap();
    // writeln!(file, "'{}'", player);

    reader.read_line(&mut gz).unwrap();
    // writeln!(file, "'{}'", gz);

    let mut grid_size = String::new();
    gz = gz.replace(":", "");
    //  gz = gz.replace(" ", "");
    gz = gz.replace("\n", "");
    grid_size = gz.trim().to_string().split(" ").collect::<Vec<&str>>()[2].to_string();
    // writeln!(file, "== grid_size: {}", grid_size);

    length_engine_grid = grid_size.trim().parse::<u32>().unwrap();
    // writeln!(file, "== length_engine_grid: {}", length_engine_grid);

    for _i in 0..length_engine_grid {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        // writeln!(file, "'{}'", line);
        // writeln!(file, "line: {}", line.trim());
        engine_output.push_str(&line);
    }
    reader.read_line(&mut engine_output).unwrap();
    return engine_output;
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

pub fn get_grid(){
    
}

