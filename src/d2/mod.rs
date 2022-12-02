use std::fs;

struct Round {
    p1: i32,
    p2: i32
}

fn as_round(s: String) -> Round {
    let player1 = (s.chars().nth(0).expect("error") as i32) - ('A' as i32);
    let player2 = (s.chars().nth(2).expect("error") as i32) - ('X' as i32);
    return Round{p1: player1, p2: player2};
}

fn get_input() -> Vec<Round> {
    let file_contents = fs::read_to_string("res/d2.txt").expect("Could not read file");
    return file_contents.lines()
        .map(|x| as_round(x.to_string()))
        .collect();
}

fn calc_score(p1: i32, p2: i32) -> i32 {
    let extra = 
    if p1 == p2 { 3 } 
    else if p2 == (p1 + 1) % 3 { 6 } 
    else { 0 };
    return p2 + extra + 1;
}

fn calc_unknown_score(p1: i32, p2: i32) -> i32 {
    let real_p2 =
    match p2 {
       0 => (p1 + 2) % 3,
       1 => p1,
       2 => (p1 + 1) % 3,
       _ => -1
    };
    return calc_score(p1, real_p2);
}

pub fn score() -> i32 {
    let rounds = get_input();
    return rounds.iter()
        .map(|x| calc_score(x.p1, x.p2))
        .sum();
}

pub fn unknown_response_score() -> i32 {
    let rounds = get_input();
    return rounds.iter()
        .map(|x| calc_unknown_score(x.p1, x.p2))
        .sum();
}