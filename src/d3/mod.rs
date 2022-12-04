use std::{fs, collections::HashSet, slice::Chunks};

fn get_input() -> Vec<String> {
    return fs::read_to_string("res/d3.txt")
        .expect("Could not read file")
        .lines()
        .map(|x| x.to_string())
        .collect()
}

fn get_char_val(c: char) -> i32 {
    return if c.is_uppercase() {
        (c as i32) - ('A' as i32) + 27
    } else {
        (c as i32) - ('a' as i32) + 1
    }
}

fn get_duplicate(rucksac: String) -> i32 {
    let first_half: HashSet<char> = rucksac[0..rucksac.len() / 2].chars().collect();
    let second_half = &rucksac[rucksac.len() / 2..rucksac.len()];
    let mut duplicate = ' ';
    for c in second_half.chars() {
        if first_half.contains(&c) {
            duplicate = c;
            break;
        }
    }
    return get_char_val(duplicate);
}

fn get_duplicate_three(trio: Vec<String>) -> i32 {
    let first: HashSet<char> = trio[0].chars().collect();
    let second: HashSet<char> = trio[1].chars().collect();
    let all_contains = trio[2].chars()
        .filter(|x| first.contains(x) && second.contains(x))
        .next()
        .unwrap();
    return get_char_val(all_contains);
}

pub fn appear_in_both() -> i32 {
    return get_input().iter()
        .map(|x| get_duplicate(x.to_string()))
        .sum();
}

pub fn group_of_threes() -> i32 {
    let input = get_input();
    return (0..input.len() / 3).into_iter()
        .map(|i| input[i * 3..i * 3 + 3].to_vec())
        .map(|x| get_duplicate_three(x))
        .sum();
}