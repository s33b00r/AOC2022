//mod d1;
//mod d2;
//mod d3;
//mod d4;
//mod d5;
//mod d6;
//mod d7;
//mod d8;
//mod d9;
//mod d10;
mod d11;


fn main() {
    println!("1: {} \n2: {}", d11::get_inspected(20, &|x| x / 3), d11::get_inspected(10_000, &|x| x));
}

/*#[test]
fn test() {
    for _ in 0..1_000 {
        d8::get_visible();
        d8::get_highest_scenic_score();
    }
    println!("1: {} \n2: {}", d8::get_visible(), d8::get_highest_scenic_score());
}*/