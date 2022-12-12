use std::{fs, collections::{HashSet, VecDeque, HashMap}};

use priority_queue::PriorityQueue;

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coords {
    x: usize,
    y: usize
}

impl Coords {
    fn get_dist(&self, other: &Coords) -> i32 {
        return (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i32; 
    }

    fn get_neighbors(self, width: usize, height: usize) -> Vec<Coords> {
        let mut vec = Vec::new();
        if self.x > 0 { vec.push(Coords{ x: self.x - 1, y: self.y }) }
        if self.x < width - 1 { vec.push(Coords{ x: self.x + 1, y: self.y }) }
        if self.y > 0 { vec.push(Coords{ x: self.x, y: self.y - 1 }) }
        if self.y < height - 1 { vec.push(Coords{ x: self.x, y: self.y + 1 }) }
        return vec;
    }
}

fn get_input() -> (Vec<Vec<char>>, (Coords, Coords)) {
    let mut s = Coords{ x: 0, y: 0};
    let mut e = Coords{ x: 0, y: 0};
    let mut map: Vec<Vec<char>> = vec![];
    for (i, l) in fs::read_to_string("res/d12.txt").expect("Could not find file").lines().enumerate() {
        let mut row = vec![];
        for (j, c) in l.chars().enumerate() {
            let coord = Coords { x: j, y: i };
            if c == 'S' {
                s = coord;
                row.push('a');
            } else if c == 'E' {
                e = coord;
                row.push('z');
            } else {
                row.push(c);
            }
        }
        map.push(row);
    }
    (map, (s, e))
}

fn nr_of_steps(map: &Vec<Vec<char>>, s: Coords, e: Coords) -> i32 {
    let mut to_check = PriorityQueue::new();
    to_check.push((s, 0), -s.get_dist(&e));
    let mut g_score = HashMap::new();
    g_score.insert(s, 0);

    let width = map[0].len();
    let height = map.len();

    while !to_check.is_empty() {
        let ((current, s), _) = to_check.pop().unwrap();
        if current == e {
            return s;
        }

        let h = map[current.y][current.x] as i32;
        for neighbor in current.get_neighbors(width, height) {
            if map[neighbor.y][neighbor.x] as i32 - h <= 1 {
                let tenative_score = s + 1;
                if tenative_score < g_score.get(&neighbor).get_or_insert(&i32::MAX).to_owned() {
                    g_score.insert(neighbor, tenative_score);
                    to_check.push((neighbor, tenative_score), -(tenative_score + neighbor.get_dist(&e)));
                }
            }
        }
    }

    return -1;
}

pub fn steps() -> i32 {
    let (map, (s, e)) = get_input();
    nr_of_steps(&map, s, e)
}

pub fn fewest_steps() -> i32 {
    let (map, (_, e)) = get_input();
    let mut smallest = i32::MAX;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'a' {
                let s = nr_of_steps(&map, Coords{ x: j, y: i}, e);
                if s > 0 {
                    smallest = smallest.min(s);
                }
            }
        }
    }
    return smallest;
}