use std::{collections::{LinkedList, HashSet}, fs};

trait Unique {
    fn get_index_of_unique(&self, nr_of_unique: usize) -> i32;
}

impl Unique for String {
    fn get_index_of_unique(&self, nr_of_unique: usize) -> i32 {
        for (i, _) in self.char_indices() {
            if i <= nr_of_unique {
                continue;
            }
            if self[i+1-nr_of_unique..i+1].chars().collect::<HashSet<char>>().len() == nr_of_unique {
                return (i + 1) as i32;
            }
        }
        return -1;
    }
}

fn get_input() -> String {
    return fs::read_to_string("res/d6.txt").expect("Could not find file");
}

pub fn marker() -> i32 {
    return get_input().get_index_of_unique(4);
}

pub fn marker_2() -> i32 {
    return get_input().get_index_of_unique(14);
}