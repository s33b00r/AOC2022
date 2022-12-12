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
mod d12;


fn main() {
    println!("1: {} \n2: {}", d12::steps(), d12::fewest_steps());
}

/*#[test]
fn test() {
    for _ in 0..1_000 {
        d8::get_visible();
        d8::get_highest_scenic_score();
    }
    println!("1: {} \n2: {}", d8::get_visible(), d8::get_highest_scenic_score());
}*/