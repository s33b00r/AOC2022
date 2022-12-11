use std::fs;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Operator {
    Square,
    Add(i64),
    Multiply(i64)
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "new = old * old" {
            Ok(Operator::Square)
        } else if let Some(x) = s.strip_prefix("new = old * ") {
            x.parse::<i64>().map(Operator::Multiply).map_err(|_| ())
        } else if let Some(x) = s.strip_prefix("new = old + ") {
            x.parse::<i64>().map(Operator::Add).map_err(|_| ())
        } else {
            Err(())
        }
    }
}

impl Operator {
    fn do_operation(&self, item: i64, modulo: i64) -> i64 {
        return match self {
            Operator::Add(x) => (item + x) % modulo,
            Operator::Multiply(x) => (item * x) % modulo,
            Operator::Square => (item * item) % modulo,
        }
    }
}

struct Monkey {
    items: Vec<i64>,
    test: i64,
    passed: usize,
    failed: usize,
    operation: Operator
}

fn generate_monkeys() -> Vec<Monkey> {
    return fs::read_to_string("res/d11.txt").unwrap().split("\n\n")
        .map(|s| {
            let mut lines = s.lines();
            let _ = lines.next();
            Some(
                Monkey {
                items: lines.next()?.rsplit_once(':')?.1
                    .split(',').map(|s| s.trim().parse().ok())
                    .collect::<Option<_>>()?,
                operation: lines.next()?.rsplit_once(": ")?.1.parse().ok()?,
                test: lines.next()?.split_ascii_whitespace().rev().next()?.parse().ok()?,
                passed: lines.next()?.split_ascii_whitespace().rev().next()?.parse().ok()?,
                failed: lines.next()?.split_ascii_whitespace().rev().next()?.parse().ok()?,
            })
        })
        .map(|m| m.unwrap())
        .collect();
}

pub fn get_inspected(iter: usize, calm_func: &dyn Fn(i64) -> i64) -> i64 {
    let mut monkeys: Vec<Monkey> = generate_monkeys();
    let mut inspections = vec![0; monkeys.len()];
    let modulo = monkeys.iter().map(|m| m.test).fold(1, |acc, x| acc * x) as i64;

    for _ in 0..iter {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let items = monkey.items.drain(..).collect::<Vec<_>>();
            let Monkey { operation, test, passed, failed, ..  } = *monkey;

            for item in items {
                let mut val = operation.do_operation(item, modulo);
                val = calm_func(val);
                let j = if val % test == 0 {
                    passed
                } else {
                    failed
                };
                monkeys[j].items.push(val);
                inspections[i] += 1;
            }
        }
    }

    inspections.sort();
    return inspections.iter().rev().take(2).fold(1, |acc, x| acc * (*x as i64));
}