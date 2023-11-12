
fn read_piece()-> Vec<String>{

    

    
}

fn get_grid_size(size_given: String)->(u32,u32){

}










fn read_input() -> Vec<String> {
    let mut input = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        input.push(line.unwrap());
    }
    input
}

fn main() {
    let input = read_input();
    println!("{:?}", input);
}
