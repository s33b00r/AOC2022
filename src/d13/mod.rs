use std::{fs, cmp::Ordering};

#[derive(PartialEq, Eq, Clone)]
enum Item {
    Integer(i32),
    List(Vec<Item>)
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Integer(b)) => a.cmp(&vec![Self::Integer(*b)]),
            (Self::Integer(a), Self::List(b)) => vec![Self::Integer(*a)].cmp(&b),
            (Self::Integer(a), Self::Integer(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

fn parse_integer(list: &[char]) -> (Item, &[char]) {
    let mut list = list;
    let mut integer = String::new();
    loop {
        if list[0].is_ascii_digit() {
            integer.push(list[0]);
            list = &list[1..];
        } else {
            break;
        }
    }
    return (Item::Integer(integer.parse().unwrap()), list);
}

fn parse_list(list: &[char]) -> (Item, &[char]) {
    let mut list = &list[1..];
    let mut items = Vec::new();

    loop {
        match list[0] {
            ']' => break,
            ',' => list = &list[1..],
            '[' => {
                let (packet, rest) = parse_list(list);
                items.push(packet);
                list = rest;
            }
            _ => {
                let (n, rest) = parse_integer(list);
                items.push(n);
                list = rest;
            }
        }
    }
    return (Item::List(items), &list[1..]);
}

fn parse_item(s: &str) -> Item {
    return parse_list(s.chars().collect::<Vec<char>>().as_slice()).0
}

fn parse(s: String) -> Vec<(Item, Item)> {
    return s.split("\n\n")
        .map(|p|{
            let mut lines = p.lines();
            let left = lines.next().unwrap();
            let right = lines.next().unwrap();
            (parse_item(left), parse_item(right))
        })
        .collect();
}

pub fn get_right_orders() -> i32 {
    let mut sum = 0;

    for (i, (l, r)) in parse(fs::read_to_string("res/d13.txt").unwrap()).iter().enumerate() {
        if l < r {
            sum += i + 1;
        }
    }
    return sum as i32;
}

pub fn get_decoder_key() -> i32 {
    let input = fs::read_to_string("res/d13.txt").unwrap();
    let start = parse_item(&"[[2]]");
    let end = parse_item(&"[[6]]");
    let mut parsed = parse(input).iter().flat_map(|(x, y)|  vec![x.clone(), y.clone()]).collect::<Vec<Item>>();
    parsed.push(start.clone());
    parsed.push(end.clone());

    parsed.sort();
    let mut s_i = -1;
    let mut e_i = -1;

    for (i, item) in parsed.iter().enumerate() {
        if *item == start {
            s_i = (i + 1) as i32;
        }
        if *item == end {
            e_i = (i + 1) as i32;
        }
    }

    return s_i * e_i;
}