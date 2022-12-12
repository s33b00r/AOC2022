use std::{fs, collections::HashSet};

fn get_input() -> Vec<Vec<i32>> {
    return fs::read_to_string("res/d08.txt").expect("Could not read file")
        .lines()
        .map(|x| x.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect::<Vec<Vec<i32>>>();
}

fn get_scenic_score(x: usize, y: usize, grid: &Vec<Vec<i32>>) -> i32 {
    let size = grid.len();
    if x == 0 || y == 0 || x == size - 1 || y == size - 1 {
        return 0;
    }
    let height = grid[y][x];
    let (mut s_l, mut s_r, mut s_u, mut s_d) = (1, 1, 1, 1);
    let mut change = true;

    while change {
        change = false;
        if x - s_l > 0 && grid[y][x - s_l] < height {
            s_l += 1;
            change = true;
        }
        if x + s_r < size - 1 && grid[y][x + s_r] < height {
            s_r += 1;
            change = true;
        }
        if y - s_u > 0 && grid[y - s_u][x] < height {
            s_u += 1;
            change = true;
        }
        if y + s_d < size - 1 && grid[y + s_d][x] < height {
            s_d += 1;
            change = true;
        }
    }

    return (s_l * s_r * s_u * s_d) as i32;
}

pub fn get_visible() -> i32 {
    let input = get_input();
    let mut set: HashSet<(usize, usize)> = HashSet::new();
    let size = input.len();

    for i in 0..size {
        let (mut largest_left, mut largest_right, mut largest_up, mut largest_down) = (-1, -1, -1, -1);
        for j in 0..size {
            if input[i][j] > largest_left {
                largest_left = input[i][j];
                set.insert((j, i));
            }

            if input[i][size - j - 1] > largest_right {
                largest_right = input[i][size - j - 1];
                set.insert((size - j - 1, i));
            }

            if input[j][i] > largest_up {
                largest_up = input[j][i];
                set.insert((i, j));
            }

            if input[size - j - 1][i] > largest_down {
                largest_down = input[size - j - 1][i];
                set.insert((i, size - j - 1));
            }
        }
    }
    return set.len() as i32;
}

pub fn get_highest_scenic_score() -> i32 {
    let input = get_input();
    let size = input.len();
    let mut highest = 0;
    for i in 0..size {
        for j in 0..size {
            highest = highest.max(get_scenic_score(j, i, &input));
        }
    }
    return highest;
}