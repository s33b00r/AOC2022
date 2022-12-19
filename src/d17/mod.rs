use std::{collections::{HashMap, HashSet}, fs, borrow::BorrowMut};

enum Shape {
    HLine{x: i32, y: i32},
    Plus{x: i32, y: i32},
    Corner{x: i32, y: i32},
    VLine{x: i32, y: i32},
    Square{x: i32, y: i32}
}

enum Direction {
    Left,
    Right,
    Down
}

impl Direction {
    fn from_char(c: char) -> Direction {
        return match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!()
        }
    }
}

impl Shape {
    fn add_to_matrix(&self, matrix: &mut HashSet<(i32, i32)>) {
        match self {
            Shape::HLine {x, y} => {
                matrix.insert((*x, *y));
                matrix.insert((*x + 1, *y));
                matrix.insert((*x + 2, *y));
                matrix.insert((*x + 3, *y));
            }
            Shape::Plus {x, y} => {
                matrix.insert((*x, *y));
                matrix.insert((*x - 1, *y - 1));
                matrix.insert((*x, *y - 1));
                matrix.insert((*x + 1, *y - 1));
                matrix.insert((*x, *y - 2));
            },
            Shape::Corner {x, y} => {
                matrix.insert((*x, *y));
                matrix.insert((*x, *y - 1));
                matrix.insert((*x, *y - 2));
                matrix.insert((*x - 1, *y - 2));
                matrix.insert((*x - 2, *y - 2));
            },
            Shape::VLine {x, y} => {
                matrix.insert((*x, *y));
                matrix.insert((*x, *y - 1));
                matrix.insert((*x, *y - 2));
                matrix.insert((*x, *y - 3));
            },
            Shape::Square {x, y} => {
                matrix.insert((*x, *y));
                matrix.insert((*x, *y - 1));
                matrix.insert((*x + 1, *y));
                matrix.insert((*x + 1, *y - 1));
            }
        }
    }
    
    fn get_y(&self) -> i32 {
        return match self {
            Shape::HLine {x, y} => *y,
            Shape::Plus {x, y} => *y,
            Shape::Corner {x, y} => *y,
            Shape::VLine {x, y} => *y,
            Shape::Square {x, y} => *y
        }
    }

    fn spawn_next(&self, highest: i32) -> Shape {
        return match self {
            Shape::HLine {..} => Shape::Plus { x: 3, y: highest + 6 },
            Shape::Plus {..} => Shape::Corner { x: 4, y: highest + 6 },
            Shape::Corner {..} => Shape::VLine { x: 2, y: highest + 7 },
            Shape::VLine {..} => Shape::Square { x: 2, y: highest + 5 },
            Shape::Square {..} => Shape::HLine { x: 2, y: highest + 4 }
        }
    }

    fn move_shape(&mut self, direction: Direction, matrix: &HashSet<(i32, i32)>) -> bool {
        match self {
            Self::HLine { x, y } => {
                match direction {
                    Direction::Left => {
                        if *x == 0 || matrix.contains(&(*x - 1, *y)) { return false; }
                        *x = *x - 1;
                        return true;
                    },
                    Direction::Right => {
                        if *x == 3 || matrix.contains(&(*x + 4, *y)) { return false; }
                        *x = *x + 1;
                        return true;
                    },
                    Direction::Down => {
                        if *y == 0 || matrix.contains(&(*x, *y - 1)) || matrix.contains(&(*x + 1, *y - 1)) ||
                            matrix.contains(&(*x + 2, *y - 1)) || matrix.contains(&(*x + 3, *y - 1)) { return false; }
                        *y = *y - 1;
                        return true;
                    }
                }
            },
            Self::Plus { x, y } => {
                match direction {
                    Direction::Left => {
                        if *x == 1 || matrix.contains(&(*x - 1, *y)) || matrix.contains(&(*x - 2, *y - 1)) || 
                            matrix.contains(&(*x - 1, *y - 2)) { return false; }
                        *x = *x - 1;
                        return true;
                    },
                    Direction::Right => {
                        if *x == 5 || matrix.contains(&(*x + 1, *y)) || matrix.contains(&(*x + 2, *y - 1)) || 
                            matrix.contains(&(*x + 1, *y - 2)) { return false; }
                        *x = *x + 1;
                        return true;
                    },
                    Direction::Down => {
                        if *y - 2 == 0 || matrix.contains(&(*x - 1, *y - 2)) || matrix.contains(&(*x, *y - 3)) ||
                            matrix.contains(&(*x + 1, *y - 2)) { return false; }
                        *y = *y - 1;
                        return true;
                    }
                }
            },
            Self::Corner { x, y } => {
                match direction {
                    Direction::Left => {
                        if *x == 2 || matrix.contains(&(*x - 1, *y)) || matrix.contains(&(*x - 1, *y - 1)) || 
                            matrix.contains(&(*x - 3, *y - 2)) { return false; }
                        *x = *x - 1;
                        return true;
                    },
                    Direction::Right => {
                        if *x == 6 || matrix.contains(&(*x + 1, *y)) || matrix.contains(&(*x + 1, *y - 1)) || 
                            matrix.contains(&(*x + 1, *y - 2)) { return false; }
                        *x = *x + 1;
                        return true;
                    },
                    Direction::Down => {
                        if *y - 2 == 0 || matrix.contains(&(*x - 2, *y - 3)) || matrix.contains(&(*x - 1, *y - 3)) ||
                            matrix.contains(&(*x, *y - 3)) { return false; }
                        *y = *y - 1;
                        return true;
                    }
                }
            },
            Self::VLine { x, y } => {
                match direction {
                    Direction::Left => {
                        if *x == 0 || matrix.contains(&(*x - 1, *y)) || matrix.contains(&(*x - 1, *y - 1)) ||
                            matrix.contains(&(*x - 1, *y - 2)) || matrix.contains(&(*x - 1, *y - 3)) { return false; }
                        *x = *x - 1;
                        return true;
                    },
                    Direction::Right => {
                        if *x == 6 || matrix.contains(&(*x + 1, *y)) || matrix.contains(&(*x + 1, *y - 1)) ||
                            matrix.contains(&(*x + 1, *y - 2)) || matrix.contains(&(*x + 1, *y - 3)) { return false; }
                        *x = *x + 1;
                        return true;
                    },
                    Direction::Down => {
                        if *y - 3 == 0 || matrix.contains(&(*x, *y - 4)) { return false; }
                        *y = *y - 1;
                        return true;
                    }
                }
            },
            Self::Square { x, y } => {
                match direction {
                    Direction::Left => {
                        if *x == 0 || matrix.contains(&(*x - 1, *y)) || matrix.contains(&(*x - 1, *y - 1)) { return false; }
                        *x = *x - 1;
                        return true;
                    },
                    Direction::Right => {
                        if *x == 5 || matrix.contains(&(*x + 2, *y)) || matrix.contains(&(*x + 2, *y - 1)) { return false; }
                        *x = *x + 1;
                        return true;
                    },
                    Direction::Down => {
                        if *y - 1 == 0 || matrix.contains(&(*x, *y - 2)) || matrix.contains(&(*x + 1, *y - 2)) { return false; }
                        *y = *y - 1;
                        return true;
                    }
                }
            }
        }
    }
}

fn get_height_diff(n: usize) -> Vec<i32> {
    let wind_dir = fs::read_to_string("res/d17.txt").unwrap().chars().collect::<Vec<_>>();
    let wind_dir_len = wind_dir.len();
    let mut wind_i = 0;
    let mut max_height = 0;
    let mut cur_rock = Shape::HLine { x: 2, y: 3 }; 
    let mut matrix = HashSet::new();

    let mut prev = 0;
    let mut diff_vec = vec![];
    for _ in 0..n {
        loop {
            let dir = Direction::from_char(wind_dir[wind_i % wind_dir_len]);
            cur_rock.move_shape(dir, &matrix);
            wind_i += 1;
            if !cur_rock.move_shape(Direction::Down, &matrix) {
                cur_rock.add_to_matrix(matrix.borrow_mut());
                max_height = max_height.max(cur_rock.get_y());
                cur_rock = cur_rock.spawn_next(max_height);
                break;
            }
        }
        diff_vec.push(max_height - prev);
        prev = max_height;
    }
    return diff_vec;
}

pub fn height_of_rocks() -> i32 {
    return get_height_diff(2022).iter().sum::<i32>() + 1;
}

pub fn height_of_rocks_2() -> i64 {
    let diffs = get_height_diff(10_000);
    let mut start_i = 0;
    let mut end_i = 0;

    for i in 0..diffs.len()-2 {
        let mut was_cycle = true;
        for j in i+1..diffs.len()-1 {
            let diff = j - i;
            if diff + j + 1 >= diffs.len() { break; }
            was_cycle = true;
            for k in j+1..diffs.len() {
                if diffs[k] != diffs[i + (k % diff)] { 
                    was_cycle = false;
                    break; 
                }
            }
            if was_cycle {
                println!("{} - {}", i, j);
                start_i = i;
                end_i = j;
                break;
            }
        }
        if was_cycle {
            break;
        }
    }

    let multiplier = 1_000_000_000_000 / (end_i - start_i) as i64 - 1;
    let extra = 1_000_000_000_000 - start_i as i64 - 1 - multiplier * (end_i - start_i) as i64;

    let h_start = diffs[0..=start_i].iter().sum::<i32>();
    let h_end = diffs[0..=end_i].iter().sum::<i32>();
    let h_extra = diffs[0..=(start_i+extra as usize)].iter().sum::<i32>();

    println!("{} -> {} + {}", h_start, h_end, h_extra);
    return (h_end - h_start) as i64 * multiplier + h_extra as i64 + 1;
}