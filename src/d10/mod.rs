use std::fs;

fn get_x_reg_values() -> Vec<i32> {
    let mut x = vec![1; 1];
    for l in fs::read_to_string("res/d10.txt").expect("Could not find file").lines() {
        match l.split_whitespace().collect::<Vec<_>>().as_slice() {
            ["noop"] => x.push(*x.last().unwrap()),
            ["addx", v] => {
                let last = *x.last().unwrap();
                x.push(last);
                x.push(last + v.parse::<i32>().unwrap());
            },
            _ => panic!()
        }
    }
    return x;
}

pub fn get_signal_strength() -> i32 {
    return get_x_reg_values().iter().enumerate()
        .filter(|(i, _)| (i + 21) % 40 == 0)
        .map(|(i, v)| (i + 1) as i32 * v)
        .sum();
}

pub fn write_to_screen() -> String {
    let mut screen = String::new();
    let x = get_x_reg_values();
    for (i, v) in x.iter().enumerate() {
        if i % 40 == 0 {
            screen.push('\n');
        }
        if ((i as i32) % 40 - v).abs() < 2 {
            screen.push('#');
        } else {
            screen.push('.');
        }
    }
    return screen;
}