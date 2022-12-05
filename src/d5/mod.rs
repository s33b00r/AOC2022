use std::{collections::LinkedList, fs};

fn get_starting_crates() -> Vec<LinkedList<char>> {
    let input = fs::read_to_string("res/d5.txt").expect("Could not find file");
    let split_i = input.find("\n\n").unwrap();

    let cols = (input[0..split_i].lines().last().unwrap().len() - 1) / 4 + 1;
    let mut crates: Vec<LinkedList<char>> = Vec::new();
    for _ in 0..cols {
        crates.push(LinkedList::new());
    }

    for line in input[0..split_i].lines() {
        for (i, c) in line.char_indices() {
            if c.is_ascii_alphabetic() {
                let index = (i - 1) / 4;
                crates.get_mut(index).unwrap().push_front(c);
            }
        }
    }
    return crates;
}

fn get_instructions() -> Vec<(i32, usize, usize)> {
    let input = fs::read_to_string("res/d5.txt").expect("Could not find file");
    let split_i = input.find("\n\n").unwrap();

    return input[split_i + 2..input.len()].lines()
        .map(|l| l.split(' ').filter_map(|s| s.parse::<i32>().ok()))
        .map(|x| x.collect::<Vec<i32>>())
        .map(|x| (x.get(0).unwrap().to_owned(), x.get(1).unwrap().to_owned() as usize, x.get(2).unwrap().to_owned() as usize))
        .collect::<Vec<(i32, usize, usize)>>();
}

pub fn upper_crates() -> String {
    let mut crates = get_starting_crates();
    let instructions = get_instructions();

    for instruction in instructions {
        for _ in 0..instruction.0 {
            let c = crates.get_mut(instruction.1 - 1).unwrap().pop_back().unwrap();
            crates.get_mut(instruction.2 - 1).unwrap().push_back(c);
        }
    }

    return crates.iter()
        .map(|x| x.back().unwrap())
        .collect();
}

pub fn upper_crates_2() -> String {
    let mut crates = get_starting_crates();
    let instructions = get_instructions();

    for instruction in instructions {
        let mut to_move = String::from("");
        for _ in 0..instruction.0 {
            to_move.push(crates.get_mut(instruction.1 - 1).unwrap().pop_back().unwrap());
        }
        to_move.chars().rev().for_each(|c| crates.get_mut(instruction.2 - 1).unwrap().push_back(c));
    }

    return crates.iter()
        .map(|x| x.back().unwrap())
        .collect();
}