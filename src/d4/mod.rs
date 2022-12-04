use std::fs;

fn get_input() -> Vec<Vec<i32>> {
    return fs::read_to_string("res/d4.txt")
        .expect("Could not find file")
        .lines()
        .map(|x| x.to_string())
        .map(|x| x.split(&['-', ',']).map(|x| x.parse().unwrap()).collect::<Vec<i32>>())
        .collect();
}

pub fn fully_contained_pairs() -> i32 {
    return get_input().iter()
        .filter(|x| (x[0] <= x[2] && x[1] >= x[3]) || (x[0] >= x[2] && x[1] <= x[3]))
        .collect::<Vec<&Vec<i32>>>()
        .len() as i32;
}

pub fn any_overlap() -> i32 {
    return get_input().iter()
        .filter(|x| (x[0] >= x[2] && x[0] <= x[3]) || (x[1] >= x[2] && x[1] <= x[3]) || (x[0] <= x[2] && x[1] >= x[3]))
        .collect::<Vec<&Vec<i32>>>()
        .len() as i32;
}