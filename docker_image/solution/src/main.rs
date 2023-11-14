use std::io;

fn read_output() {
    // Premier joueur
    let mut player1_name = String::new();
    io::stdin().read_line(&mut player1_name)
        .expect("Failed to read line");
    println!("Player 1: {}", player1_name.trim());

    // Deuxième joueur
    let mut player2_name = String::new();
    io::stdin().read_line(&mut player2_name)
        .expect("Failed to read line");
    println!("Player 2: {}", player2_name.trim());
}

fn main() {
    read_output();
}
