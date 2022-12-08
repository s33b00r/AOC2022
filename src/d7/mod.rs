use std::{fs, ops::Add, collections::{HashMap, VecDeque}};

fn get_dir_sizes() -> HashMap<String, i32> {
    let input = fs::read_to_string("res/d7.txt").expect("Could not find file");

    let mut size_map: HashMap<String, i32> = HashMap::new();
    let mut dir_stack: VecDeque<String> = VecDeque::new();

    for line in input.lines() {

        match line.split_whitespace()
            .collect::<Vec<_>>()
            .as_slice()
        {
            ["$", "cd", ".."] => { dir_stack.pop_front(); }
            ["$", "cd", d] => {
                let front = match dir_stack.front() { Some(x) => x.to_owned(), None => String::from("") };
                dir_stack.push_front(front + &d.to_string());
            }
            [f, _] if *f != "$" && *f != "dir" => {
                let size: i32 = f.parse().unwrap();
                for dir in dir_stack.iter() {
                    let cur_size = match size_map.get(dir) {
                        Some(x) => x.to_owned(),
                        None => 0
                    };
                    size_map.insert(dir.to_owned(), cur_size + size);
                }
            }
            _ => (),
        }
    }
    return size_map;
}

pub fn get_sum() -> i32 {
    return get_dir_sizes().values()
        .filter(|&x| *x <= 100_000)
        .sum()
}

pub fn get_smallest_to_remove() -> i32 {
    let dirs = get_dir_sizes();
    let minimal_to_remove: i32 = dirs.get("/").unwrap() - 40_000_000;
    return dirs.values()
        .filter(|x| x.to_owned().to_owned() >= minimal_to_remove)
        .fold(std::i32::MAX, |acc, item| {
            if *item < acc { *item } else { acc }
        });
}