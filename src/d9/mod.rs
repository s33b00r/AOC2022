use std::{fs, collections::HashSet};

fn get_input() -> Vec<Vec<String>>{
    return fs::read_to_string("res/d9.txt").expect("Could not find file")
        .lines()
        .map(|l| l.split_whitespace().map(|s| s.to_string()).collect())
        .collect();
}

fn get_delta(dir: &String) -> (i32, i32) {
    return match dir.as_str() {
        "U" => (0, -1),
        "D" => (0, 1),
        "R" => (1, 0),
        "L" => (-1, 0),
        _ => panic!()
    };
}

fn next_pos(ahead: (i32, i32), current: (i32, i32)) -> (i32, i32) {
    let dx_tail = ahead.0 - current.0;
    let dy_tail = ahead.1 - current.1;
    
    return match (dx_tail.abs(), dy_tail.abs()) {
        (2, 0) => (current.0 + dx_tail / 2, current.1),
        (2, 1) => (current.0 + dx_tail / 2, current.1 + dy_tail),
        (1, 2) => (current.0 + dx_tail, current.1 + dy_tail / 2),
        (0, 2) => (current.0, current.1 + dy_tail / 2),
        (2, 2) => (current.0 + dx_tail / 2, current.1 + dy_tail / 2),
        _ => current
    }
}

pub fn get_nr_tail_pos() -> i32 {
    let mut tail_pos: HashSet<(i32, i32)> = HashSet::new();
    let mut cur_head = (0, 0);
    let mut cur_tail = (0, 0);
    tail_pos.insert(cur_tail);

    for step in get_input() {
        let (dx, dy) = get_delta(&step[0]);
        let s: i32 = step[1].parse().unwrap();
        for _ in 0..s {
            cur_head = (cur_head.0 + dx, cur_head.1 + dy);
            cur_tail = next_pos(cur_head, cur_tail);
            tail_pos.insert(cur_tail);
        }
    }
    return tail_pos.len() as i32;
}

pub fn get_nr_tail_pos_2() -> i32 {
    let mut tail_pos: HashSet<(i32, i32)> = HashSet::new();
    tail_pos.insert((0, 0));
    let mut ropes = [(0, 0); 10];
    for step in get_input() {
        let (dx, dy) = get_delta(&step[0]);
        let s: i32 = step[1].parse().unwrap();
        for _ in 0..s {
            ropes[0] = (ropes[0].0 + dx, ropes[0].1 + dy);
            for i in 1..ropes.len() {
                ropes[i] = next_pos(ropes[i - 1], ropes[i]);
            }
            tail_pos.insert(ropes.last().unwrap().to_owned());
        }
    }

    return tail_pos.len() as i32;
}