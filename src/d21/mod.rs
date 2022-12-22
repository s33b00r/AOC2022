use std::{str::FromStr, num::ParseIntError, collections::HashMap, fs};

enum Op {
    Add,
    Subtract,
    Divide,
    Multiply
}

impl Op {
    fn do_operation(&self, first: i64, second: i64) -> i64 {
        match self {
            Self::Add => first + second,
            Self::Divide => first / second,
            Self::Multiply => first * second,
            Self::Subtract => first - second
        }
    }
}

enum Monkey {
    Number{ name: String, number: i64 },
    Operation { name: String, monkey1: String, monkey2: String, operation: Op }
}

impl Monkey {
    fn get_name(&self) -> String {
        match self {
            Self::Number { name, ..} => name.to_string(),
            Self::Operation { name, ..} => name.to_string()
        }
    }
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<_>>().as_slice() {
            [name, number] => {
                return Ok(Self::Number { name: name.strip_suffix(":").unwrap().to_string(), number: number.parse::<i64>()? });
            },
            [name, m1, op, m2] => {
                let operation = match *op {
                    "+" => Op::Add,
                    "-" => Op::Subtract,
                    "*" => Op::Multiply,
                    "/" => Op::Divide,
                    _ => panic!()
                };
                return Ok(Self::Operation { name: name.strip_suffix(":").unwrap().to_string(), monkey1: m1.to_string(), monkey2: m2.to_string(), operation: operation });
            },
            _ => panic!()
        }
    }
}

fn get_input() -> HashMap<String, Monkey> {
    fs::read_to_string("res/d21.txt").unwrap().lines()
        .map(|l| l.parse::<Monkey>().unwrap())
        .map(|m| (m.get_name(), m))
        .collect()
}

fn get_number(monkey: &String, monkeys: &HashMap<String, Monkey>) -> i64 {
    match monkeys.get(monkey).unwrap() {
        Monkey::Number { number, .. } => *number,
        Monkey::Operation { monkey1, monkey2, operation, .. } => {
            operation.do_operation(get_number(monkey1, monkeys), get_number(monkey2, monkeys))
        }
    }
}

fn get_wanted_number(wanted_monkey: &String, current: &String, wanted_number: i64, monkeys: &HashMap<String, Monkey>) -> i64 {
    if current == wanted_monkey { return wanted_number; }
    return match monkeys.get(current).unwrap() {
        Monkey::Operation {monkey1, monkey2, operation, .. } => {
            if contains(monkey1, wanted_monkey, monkeys) {
                let other_branch = get_number(monkey2, monkeys);
                match operation {
                    Op::Add => get_wanted_number(wanted_monkey, monkey1, wanted_number - other_branch, monkeys),
                    Op::Subtract => get_wanted_number(wanted_monkey, monkey1, wanted_number + other_branch, monkeys),
                    Op::Multiply => get_wanted_number(wanted_monkey, monkey1, wanted_number / other_branch, monkeys),
                    Op::Divide => get_wanted_number(wanted_monkey, monkey1, wanted_number * other_branch, monkeys)
                }
            } else {
                let other_branch = get_number(monkey1, monkeys);
                match operation {
                    Op::Add => get_wanted_number(wanted_monkey, monkey2, wanted_number - other_branch, monkeys),
                    Op::Subtract => get_wanted_number(wanted_monkey, monkey2, other_branch - wanted_number, monkeys),
                    Op::Multiply => get_wanted_number(wanted_monkey, monkey2, wanted_number / other_branch, monkeys),
                    Op::Divide => get_wanted_number(wanted_monkey, monkey2, other_branch / wanted_number, monkeys)
                }
            }
        }
        _ => panic!()
    }
}

fn contains(current: &String, wanted: &String, monkeys: &HashMap<String, Monkey>) -> bool {
    match monkeys.get(current).unwrap() {
        Monkey::Number { name, ..} => name == wanted,
        Monkey::Operation { name, monkey1, monkey2, ..} => {
            name == wanted || contains(monkey1, wanted, monkeys) || contains(monkey2, wanted, monkeys)
        }
    }
}

pub fn get_roots_number() -> i64 {
    return get_number(&"root".to_string(), &get_input())
}

pub fn get_humns_number() -> i64 {
    let monkeys = get_input();
    let root = monkeys.get(&"root".to_string()).unwrap();
    match root {
        Monkey::Operation { monkey1, monkey2, ..} => {
            if contains(monkey1, &"humn".to_string(), &monkeys) {
                let wanted = get_number(monkey2, &monkeys);
                return get_wanted_number(&"humn".to_string(), monkey1, wanted, &monkeys);
            } else {
                let wanted = get_number(&monkey1, &monkeys);
                return get_wanted_number(&"humn".to_string(), monkey2, wanted, &monkeys);
            }
        },
        _ => panic!()
    };
}