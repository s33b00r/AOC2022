use std::{fs, borrow::BorrowMut};

fn get_input() -> Vec<i64> {
    return fs::read_to_string("res/d20.txt").unwrap().lines()
        .map(|l| l.parse().unwrap())
        .collect();
}

fn mix(items: &mut Vec<(i64, usize)>) {
    for real_i in 0..items.len() {
        let mut i = usize::MAX;
        for current_i in 0..items.len() {
            if items[current_i].1 == real_i {
                i = current_i;
                break;
            }
        }

        let tmp = items.remove(i).0;
        let mut new_i = i as i64 + tmp;
        new_i = new_i % items.len() as i64;
        new_i = (new_i + items.len() as i64) % items.len() as i64;
        if new_i == 0 {
            items.push((tmp, real_i));
        } else {
            items.insert(new_i as usize, (tmp, real_i));
        }
    }
}

pub fn get_decrypted_coords(multiplier: i64, times_to_mix: usize) -> i64 {
    let mut scrambled_coords = get_input().iter().enumerate()
        .map(|(i, x)| (*x * multiplier, i))
        .collect::<Vec<_>>();

    for _ in 0..times_to_mix {
        mix(scrambled_coords.borrow_mut());
    }

    let mut zero_i = 0;
    for i in 0..scrambled_coords.len() {
        if scrambled_coords[i].0 == 0 {
            zero_i = i;
            break;
        }
    }

    return scrambled_coords[(1000 + zero_i) % scrambled_coords.len()].0 + scrambled_coords[(2000 + zero_i) % scrambled_coords.len()].0 + scrambled_coords[(3000 + zero_i) % scrambled_coords.len()].0;
}