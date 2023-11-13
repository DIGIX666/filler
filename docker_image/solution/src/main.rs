mod piece;
mod tools;

use piece::read_piece;
use tools::read_line;


fn main() {

   println!("PIECE: {:?}",read_piece(read_line())) 
    
}
