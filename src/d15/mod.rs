use std::{collections::HashSet, fs, ops::{Add, AddAssign}};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, other: Self) -> Self::Output {
        Coord { x: self.x + other.x, y: self.y + other.y }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Coord {
    fn manhattan_dist(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn get_input() -> Vec<(Coord, Coord)> {
    fs::read_to_string("res/d15.txt").unwrap().lines()
        .map(|l| {
            let splitted = l.split_once(": ").unwrap();
            let sensor = splitted.0.strip_prefix("Sensor at ").unwrap().split_once(", ").unwrap();
            let sensor_coord = Coord{ x: sensor.0.strip_prefix("x=").unwrap().parse().unwrap(), y: sensor.1.strip_prefix("y=").unwrap().parse().unwrap() };
            let beacon = splitted.1.strip_prefix("closest beacon is at ").unwrap().split_once(", ").unwrap();
            let beacon_coord = Coord{ x: beacon.0.strip_prefix("x=").unwrap().parse().unwrap(), y: beacon.1.strip_prefix("y=").unwrap().parse().unwrap() };
            (sensor_coord, beacon_coord)
        })
        .collect()
}

pub fn missing_in_row(y: i32) -> i32 {
    let mut row_lookups = HashSet::new();

    for (sensor, beacon) in get_input() {
        let dist = sensor.manhattan_dist(&beacon);
        let d = dist - (sensor.y - y).abs();
        if d < 0 { continue; }
        (sensor.x-d..=sensor.x+d).for_each(|x| { row_lookups.insert(Coord{x, y}); })
    }
    return (row_lookups.len() - get_input().iter().map(|x| x.1.clone()).filter(|c| c.y == y).collect::<HashSet<Coord>>().len()) as i32;
}

pub fn missing_beacon(max: i32) -> i64 {
    let input = get_input().iter()
        .map(|x| (x.0.clone(), x.0.manhattan_dist(&x.1)))
        .collect::<Vec<(Coord, i32)>>();
    let mut edges = HashSet::new();
    for (c, d) in input[0..].iter() {
        let mut up_right = Coord { x: c.x - d - 1, y: c.y };
        let mut down_right = Coord { x: c.x, y: c.y - d - 1 };
        let mut down_left = Coord { x: c.x + d + 1, y: c.y };
        let mut up_left = Coord { x: c.x, y: c.y + d + 1 };
        for _ in 0..=*d {
            edges.insert(up_right);
            edges.insert(down_right);
            edges.insert(down_left);
            edges.insert(up_left);
            up_right += Coord { x: 1, y: -1};
            down_right += Coord { x: 1, y: 1};
            down_left += Coord { x: -1, y: 1};
            up_left += Coord { x: -1, y: -1};
        }
        edges.insert(up_right);
        edges.insert(down_right);
        edges.insert(down_left);
        edges.insert(up_left);
    }
    edges = edges.iter().filter(|c| c.x > 0 && c.x <= max && c.y > 0 && c.y <= max).map(|c| *c).collect();
    for edge in edges {
        let mut has_coverage = false;
        for (c, d) in input[0..].iter() {
            if c.manhattan_dist(&edge) <= *d {
                has_coverage = true;
                break;
            }
        }
        if !has_coverage {
            return (edge.x as i64 * 4_000_000) + edge.y as i64;
        }
    }
    return -1;
}