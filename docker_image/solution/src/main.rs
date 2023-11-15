use std::io;

fn read_output() {
    // Premier joueur
    let mut player1_name = String::new();
    io::stdin().read_line(&mut player1_name)
        .expect("Failed to read line");
    println!("{}", player1_name.trim());

    // Deuxième joueur
    let mut player2_name = String::new();
    io::stdin().read_line(&mut player2_name)
        .expect("Failed to read line");
    println!("{}", player2_name.trim());
    let mut input = String::new();
    let mut i = 0;
    loop{

        let mut player1_name = String::new();
        io::stdin().read_line(&mut player1_name)
            .expect("Failed to read line");
        println!("Player 1: {}", player1_name);
        input += &player1_name; 
        i+=1;
    }

    // // Deuxième joueur
    // let mut player2_name = String::new();
    // io::stdin().read_line(&mut player2_name)
    //     .expect("Failed to read line");
    println!("input: {}", input);
}

fn main() {
    read_output();
}
