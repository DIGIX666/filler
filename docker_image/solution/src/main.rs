use std::io;

fn main() {
    read_output();
}

fn read_output() {
    let mut playername = String::new();
    io::stdin().read_line(&mut playername)
        .expect("Failed to read line");
        println!("{}", playername);
}