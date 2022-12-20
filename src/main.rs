//mod d01;
//mod d02;
//mod d03;
//mod d04;
//mod d05;
//mod d06;
//mod d07;
//mod d08;
//mod d09;
//mod d10;
//mod d11;
//mod d12;
//mod d13;
//mod d14;
//mod d15;
//mod d16;
//mod d17;
//mod d18;
//mod d19;
mod d20;



fn main() {
    println!("1: {} \n2: {}", d20::get_decrypted_coords(1, 1), d20::get_decrypted_coords(811589153, 10));
}

/*#[test]
fn test() {
    for _ in 0..1_000 {
        d8::get_visible();
        d8::get_highest_scenic_score();
    }
    println!("1: {} \n2: {}", d8::get_visible(), d8::get_highest_scenic_score());
}*/