use std::{collections::HashSet, fs};

fn get_input() -> HashSet<(i32, i32, i32)> {
    return fs::read_to_string("res/d18.txt").unwrap().lines()
        .map(|l| {
            let mut split = l.split(',');
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            let z = split.next().unwrap().parse().unwrap();
            (x, y, z)
        })
        .collect();
}

fn get_area(cubes: &HashSet<(i32, i32, i32)>) -> i32 {
    let mut visible_sides = 0;
    for (x, y, z) in cubes.clone() {
        if !cubes.contains(&(x - 1, y, z)) { visible_sides += 1; }
        if !cubes.contains(&(x + 1, y, z)) { visible_sides += 1; }
        if !cubes.contains(&(x, y - 1, z)) { visible_sides += 1; }
        if !cubes.contains(&(x, y + 1, z)) { visible_sides += 1; }
        if !cubes.contains(&(x, y, z - 1)) { visible_sides += 1; }
        if !cubes.contains(&(x, y, z + 1)) { visible_sides += 1; }
    }
    return visible_sides;
}

pub fn get_surface_area() -> i32 {
    return get_area(&get_input());
}

fn can_get_out(x: i32, y: i32, z: i32, min: i32, max: i32, cubes: &HashSet<(i32, i32, i32)>) -> bool {

    let mut possible = HashSet::new();
    possible.insert((x, y, z));
    let mut seen = HashSet::new();
    loop {
        if possible.len() == 0 { return false; }
        let mut new_possible = HashSet::new();
        for (n_x, n_y, n_z) in possible.clone() {
            if n_x == min || n_x == max || n_y == min || n_y == max || n_z == min || n_z == max { return true; }
            seen.insert((n_x, n_y, n_z));
            let l = (n_x - 1, n_y, n_z);
            let r = (n_x + 1, n_y, n_z);
            let u = (n_x, n_y + 1, n_z);
            let d = (n_x, n_y - 1, n_z);
            let f = (n_x, n_y, n_z + 1);
            let b = (n_x, n_y, n_z - 1);
            if !seen.contains(&l) && !cubes.contains(&l) { new_possible.insert(l); }
            if !seen.contains(&r) && !cubes.contains(&r) { new_possible.insert(r); }
            if !seen.contains(&u) && !cubes.contains(&u) { new_possible.insert(u); }
            if !seen.contains(&d) && !cubes.contains(&d) { new_possible.insert(d); }
            if !seen.contains(&f) && !cubes.contains(&f) { new_possible.insert(f); }
            if !seen.contains(&b) && !cubes.contains(&b) { new_possible.insert(b); }
        }
        possible = new_possible;
    }
}

pub fn get_surface_area_without_pockets() -> i32 {
    let mut cubes = get_input();
    let mut min = 0;
    let mut max = 0;

    for (x, y, z) in cubes.clone() {
        min = min.min(x);
        min = min.min(y);
        min = min.min(z);
        max = max.max(x);
        max = max.max(y);
        max = max.max(z);
    }

    for x in min+1..=max-1 {
        for y in min+1..=max-1 {
            for z in min+1..=max-1 {

                if cubes.contains(&(x, y, z)) { continue; }
                if !can_get_out(x, y, z, min, max, &cubes) { 
                    cubes.insert((x, y, z)); 
                }
            }
        }
    }

    return get_area(&cubes);
}