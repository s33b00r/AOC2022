use std::{collections::LinkedList, fs, str::Lines};

fn get_input() -> (Vec<LinkedList<char>>, Vec<(i32, usize, usize)>) {
    let input = fs::read_to_string("res/d05.txt").expect("Could not find file");
    let split_i = input.find("\n\n").unwrap();

    let cols = (input[0..split_i].lines().last().unwrap().len() - 1) / 4 + 1;

    fn get_column(i: usize, l: Lines) -> LinkedList<char> {
        return l.map(|x| x.chars().nth(4 * i + 1))
            .filter_map(|x| x)
            .filter(|x| x.is_ascii_alphabetic())
            .collect();
    }

    let crates: Vec<LinkedList<char>> = (0..cols)
        .map(|i| get_column(i, input[0..split_i].lines()))
        .collect();

    let instructions = input[split_i + 2..input.len()].lines()
        .map(|l| l.split(' ').filter_map(|s| s.parse::<i32>().ok()))
        .map(|x| x.collect::<Vec<i32>>())
        .map(|x| (x.get(0).unwrap().to_owned(), x.get(1).unwrap().to_owned() as usize, x.get(2).unwrap().to_owned() as usize))
        .collect::<Vec<(i32, usize, usize)>>();

    return (crates, instructions);
}

pub fn upper_crates() -> String {
    let (mut crates, instructions) = get_input();

    for instruction in instructions {
        for _ in 0..instruction.0 {
            let c = crates.get_mut(instruction.1 - 1).unwrap().pop_front().unwrap();
            crates.get_mut(instruction.2 - 1).unwrap().push_front(c);
        }
    }

    return crates.iter()
        .map(|x| x.front().unwrap())
        .collect();
}

pub fn upper_crates_2() -> String {
    let (mut crates, instructions) = get_input();

    for instruction in instructions {
        let mut to_move = String::from("");
        for _ in 0..instruction.0 {
            to_move.push(crates.get_mut(instruction.1 - 1).unwrap().pop_front().unwrap());
        }
        to_move.chars().rev().for_each(|c| crates.get_mut(instruction.2 - 1).unwrap().push_front(c));
    }

    return crates.iter()
        .map(|x| x.front().unwrap())
        .collect();
}