use std::fs;

fn get_input() -> Vec<String> {
    let file_contents = fs::read_to_string("res/d01.txt").expect("Could not read file");
    return file_contents.lines().map(|x| x.to_string()).collect()
}

fn get_calories_list() -> Vec<i32> {
    let contents = get_input();
    let mut list: Vec<i32> = Vec::new();
    let mut current = 0;
    for cal in contents {
        if cal.is_empty() {
            list.push(current);
            current = 0;
            continue;
        }
        current += cal.parse::<i32>().expect("Could not parse");
    }
    list.push(current);
    return list;
}

pub fn highest_calories() -> i32 {
    let mut highest = 0;

    for cal in get_calories_list() {
        if highest < cal {
            highest = cal
        }
    }
    return highest;
}

pub fn highest_calories_times_3() -> i32 {
    let mut highest = [0; 3];
    for cal in get_calories_list() {
        if highest[0] < cal {
            highest[0] = cal;
            highest.sort();
        }
    }
    return highest.iter().sum();
}