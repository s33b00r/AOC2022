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
mod d15;


fn main() {
    println!("1: {} \n2: {}", d15::missing_in_row(2_000_000), d15::missing_beacon(4_000_000));
}

/*#[test]
fn test() {
    for _ in 0..1_000 {
        d8::get_visible();
        d8::get_highest_scenic_score();
    }
    println!("1: {} \n2: {}", d8::get_visible(), d8::get_highest_scenic_score());
}*/