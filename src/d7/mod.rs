use std::{fs, ops::Add, collections::{HashMap, VecDeque}};

fn get_dir_sizes() -> HashMap<String, i32> {
    let input = fs::read_to_string("res/d7.txt").expect("Could not find file");

    let mut size_map: HashMap<String, i32> = HashMap::new();
    size_map.insert(String::from("/"), 0);
    let mut dir_stack: VecDeque<String> = VecDeque::new();
    dir_stack.push_front(String::from("/"));

    for line in input.lines() {
        let mut split = line.split(' ');
        let first = split.next().unwrap().to_owned();
        let second = split.next().unwrap().to_owned();
        if first == "$" {
            if second == "cd" {
                let third = split.next().unwrap().to_owned();
                if third == "/" {
                    continue;
                }
                if third == ".." {
                    dir_stack.pop_front();
                } else {
                    dir_stack.push_front(dir_stack.front().unwrap().to_string() + &third);
                }
            }
            continue;
        }

        if !first.contains("dir") {
            let size: i32 = first.parse().unwrap();
            for dir in dir_stack.iter() {
                let cur_size = match size_map.get(dir) {
                    Some(x) => x.to_owned(),
                    None => 0
                };
                size_map.insert(dir.to_owned(), cur_size + size);
            }
        }
    }
    return size_map;
}

pub fn get_sum() -> i32 {
    return get_dir_sizes().values()
        .filter(|x| x.to_owned().to_owned() <= 100000)
        .sum()
}

pub fn get_smallest_to_remove() -> i32 {
    let dirs = get_dir_sizes();
    let minimal_to_remove: i32 = dirs.get("/").unwrap() - 40000000;
    return dirs.values()
        .filter(|x| x.to_owned().to_owned() >= minimal_to_remove)
        .fold(std::i32::MAX, |acc, item| {
            if *item < acc { *item } else { acc }
        });
}