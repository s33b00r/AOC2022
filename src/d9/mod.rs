use std::{fs, collections::HashSet};

fn get_input() -> Vec<Vec<String>>{
    return fs::read_to_string("res/d9.txt").expect("Could not find file")
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
        .collect();
}

fn move_dir(dir: &String, pos: (i32, i32)) -> (i32, i32) {
    return match dir.as_str() {
        "U" => (pos.0, pos.1 - 1),
        "D" => (pos.0, pos.1 + 1),
        "R" => (pos.0 + 1, pos.1),
        "L" => (pos.0 - 1, pos.1),
        _ => panic!()
    };
}

fn next_pos(ahead: (i32, i32), current: (i32, i32)) -> (i32, i32) {
    let dx = ahead.0 - current.0;
    let dy = ahead.1 - current.1;
    
    return match (dx.abs(), dy.abs()) {
        (2, 0) => (current.0 + dx / 2, current.1),
        (2, 1) => (current.0 + dx / 2, current.1 + dy),
        (1, 2) => (current.0 + dx, current.1 + dy / 2),
        (0, 2) => (current.0, current.1 + dy / 2),
        (2, 2) => (current.0 + dx / 2, current.1 + dy / 2),
        _ => current
    }
}

pub fn get_nr_tail_pos(knots: usize) -> i32 {
    let mut tail_pos: HashSet<(i32, i32)> = HashSet::new();
    tail_pos.insert((0, 0));
    let mut ropes = vec![(0, 0); knots];
    for step in get_input() {
        let s: i32 = step[1].parse().unwrap();
        for _ in 0..s {
            ropes[0] = move_dir(&step[0], ropes[0]);
            for i in 1..ropes.len() {
                ropes[i] = next_pos(ropes[i - 1], ropes[i]);
            }
            tail_pos.insert(ropes.last().unwrap().to_owned());
        }
    }
    return tail_pos.len() as i32;
}