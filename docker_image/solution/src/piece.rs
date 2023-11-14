
pub fn read_piece(std_input: String) -> Vec<Vec<String>> {
    let mut engine_response_vec = std_input.split("\n").collect::<Vec<&str>>();
    let engine_rep_vec = std_input.split(":").collect::<Vec<&str>>();

    let mut size_grid: &str = "";
    let mut previous_grid: &str = "";
    let mut piece: Vec<Vec<String>> = Vec::new();
    let mut piece_grid:&str="";
    let mut piece_size_string: String = "".to_string();
    let mut grid_size_string: String = "".to_string();
    let mut piece_size:Vec<u32> = Vec::new();
    let mut grid_size:Vec<u32> = Vec::new();
    let mut grid_size_vec:Vec<&str> = Vec::new();
    let mut piece_size_vec:Vec<&str> = Vec::new();

    piece_grid = engine_rep_vec[engine_rep_vec.len() - 1];
    previous_grid = engine_rep_vec[1];
    for i in 0..engine_response_vec.len() {
        //    println!("{} : {}",i,engine_response_vec[i]);

        if engine_response_vec[i].contains("Anfield") {
            grid_size_string = engine_response_vec[i].replace("Anfield ", "");
            grid_size_string = grid_size_string.replace(":", "");

        }

        if engine_response_vec[i].contains("Piece") {

            piece_size_string = engine_response_vec[i].replace("Piece ", "");
            piece_size_string = piece_size_string.replace(":", "");
            
        }
    }

    println!("Anfield size:{}", grid_size_string);
    println!("Piece size:{}", piece_size_string);

    grid_size_vec = grid_size_string.split(" ").collect::<Vec<&str>>();
    piece_size_vec = piece_size_string.split(" ").collect::<Vec<&str>>();

    for i in 0..grid_size_vec.len() {
        grid_size.push(grid_size_vec[i].to_string().parse::<u32>().unwrap());
    }

    for i in 0..piece_size_vec.len() {
        piece_size.push(piece_size_vec[i].to_string().parse::<u32>().unwrap());
    }

    // let mut grid_piece:Vec<&str> = Vec::new();

    let piece_dirty = piece_grid.clone().to_string();
    let mut piece_dirty_vec = piece_dirty.split("").collect::<Vec<&str>>();
    let mut good = false;
    let mut col: Vec<String>=Vec::new();
    
    for e in piece_dirty_vec {
        good = false;
        for (i,char) in e.chars().enumerate() {

            if char.is_ascii_alphanumeric() || char.is_ascii_punctuation() {
                good = true;
                col.push(char.to_string());
            }else{
                good = false;
            }
            
        }
        if good && col.len() == piece_size[0] as usize {
            
            piece.push(col.clone());
        }
        if !good  {
            col.clear();  
        }
       
    }

    // println!("piece : {:?}", piece);
    return piece;
}

pub fn piece_coord(piece: Vec<Vec<String>>)->(u32,u32){

    let mut numLine: u32 = 0;
    let mut numCol: u32 = 0;

    for line in piece {
        for col in line {
            if col == "O" {

                numCol+1;
            }
        }
        numLine+1;
    }



    return (numCol,numLine);

}