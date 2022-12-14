use std::{collections::HashSet, fs, ops::{Add, Sub}};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn new(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }

    fn signum(&self) -> Coord {
        return Coord::new(self.x.signum(), self.y.signum());
    }

    fn add(self, x: i32, y: i32) -> Coord {
        Coord { x: self.x + x, y: self.y + y }
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Self) -> Self::Output {
       Coord { x: self.x + other.x, y: self.y + other.y } 
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, other: Self) -> Self::Output {
       Coord { x: self.x - other.x, y: self.y - other.y } 
    }
}

fn get_input() -> Vec<Vec<Coord>> {
    return fs::read_to_string("res/d14.txt").unwrap().lines()
        .map(|s| {
            let coords = s.split(" -> ").map(|i| i.split_once(",").unwrap());
            coords.map(|c| Coord::new(c.0.parse::<i32>().unwrap(), c.1.parse::<i32>().unwrap())).collect()
        })
        .collect()
}

fn get_input_walls() -> HashSet<Coord> {
    let mut sparse_matrix = HashSet::new();

    for path in get_input() {
        for i in 1..path.len() {
            let d_coord = (path[i] - path[i - 1]).signum();
            let mut pos = path[i - 1];
            sparse_matrix.insert(pos);
            loop {
                pos = pos + d_coord;
                sparse_matrix.insert(pos);
                if pos == path[i] { break; }
            }
        }
    }
    return sparse_matrix;
}

pub fn amount_of_sand() -> i32 {
    let mut sparse_matrix = get_input_walls();

    let mut amount_sand = 0;
    loop {
        let mut pos = Coord::new(500, 0);
        let mut iter = 0;
        let mut failed = false;
        loop {
            iter += 1;

            if !sparse_matrix.contains(&pos.add(0, 1)) {
                pos = pos.add(0, 1);
            } else if !sparse_matrix.contains(&pos.add(-1, 1)) {
                pos = pos.add(-1, 1);
            } else if !sparse_matrix.contains(&pos.add(1, 1)) {
                pos = pos.add(1, 1);
            } else {
                amount_sand += 1;
                sparse_matrix.insert(pos);
                break;
            }

            if iter > 10_000 {
                failed = true;
                break;
            }
        }
        if failed { break; }
    }

    return amount_sand;
}

pub fn amount_of_sand_2() -> i32 {
    let mut sparse_matrix = get_input_walls();
    let floor = sparse_matrix.iter().map(|c| c.y).fold(0, |acc, y| acc.max(y)) + 2;

    let mut amount_sand = 0;
    loop {
        amount_sand += 1;
        let mut pos = Coord::new(500, 0);
        loop {
            if pos.y + 1 == floor {
                sparse_matrix.insert(pos);
                break;
            }
            if !sparse_matrix.contains(&pos.add(0, 1)) {
                pos = pos.add(0, 1);
            } else if !sparse_matrix.contains(&pos.add(-1, 1)) {
                pos = pos.add(-1, 1);
            } else if !sparse_matrix.contains(&pos.add(1, 1)) {
                pos = pos.add(1, 1);
            } else {
                sparse_matrix.insert(pos);
                break;
            }
        }
        if pos.x == 500 && pos.y == 0 { break; }
    }

    return amount_sand;
}