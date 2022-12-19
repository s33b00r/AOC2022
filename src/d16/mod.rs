use std::{collections::{HashMap, HashSet, VecDeque}, fs, borrow::BorrowMut};

fn get_input() -> HashMap<String, (i32, Vec<String>)>{
    let mut nodes = HashMap::new();
    fs::read_to_string("res/d16.txt").unwrap().lines().map(|l| {
        let (desc, edge_desc) = l.split_once(';').unwrap();
        let mut split = desc.split_whitespace();
        split.next();
        let name = split.next().unwrap().to_string();
        split.next();
        split.next();
        let pressure = split.next().unwrap().strip_prefix("rate=").unwrap().parse::<i32>().unwrap();
        let edges = edge_desc.split_whitespace().skip(4).map(|s| s.strip_suffix(",").get_or_insert(s).to_string()).collect::<Vec<String>>();
        (name, pressure, edges)
    })
    .for_each(|i| { nodes.insert(i.0, (i.1, i.2)); });
    return nodes;
}

fn part_pressure(dist_pressure_matrix: &Vec<Vec<(i32, i32)>>, current: u32, visited: u32, all: u32, time_remaining: i32) -> HashSet<(i32, u32)> {
    let mut available = all & !visited;
    available = available << 1;
    let mut i: i32 = -1;
    let mut potentials = vec![];
    while available != 0 {
        i += 1;
        available = available >> 1;

        if available & 1 == 0 {
            continue;
        } 

        let (d, p) = dist_pressure_matrix[current as usize][i as usize];
        if d + 1 > time_remaining { continue; }
        potentials.push((i, d, p));
    }

    if potentials.len() == 0 {
        let mut best = HashSet::new();
        best.insert((0, visited));
        return best;
    }

    let mut best_paths = vec![];
    for (n, d, p) in potentials {
        let pressure_release = p * (time_remaining - d - 1);
        let paths = part_pressure(
            dist_pressure_matrix,
            n as u32,
            visited + (1 << n) as u32, 
            all, 
            time_remaining - d - 1
        );
        paths.iter().for_each(|(r, p)| best_paths.push((*r + pressure_release, *p)));
    }

    if best_paths.len() > 100 {
        best_paths.sort_by(|a, b| b.0.cmp(&a.0));
        best_paths = best_paths[0..100].iter().map(|x| *x).collect();
    }

    return best_paths.iter().map(|x| *x).collect();
}

pub fn get_total_pressure() -> (i32, i32) {
    let input = get_input();
    let mut good_node = input.iter()
        .filter(|(_, v)| v.0 > 0)
        .map(|(k, _)| k.as_str())
        .collect::<Vec<_>>();
    good_node.push("AA");
    let mut dist_matrix = vec![vec![(0, 0); good_node.len()]; good_node.len()];
    for (i, node) in good_node.iter().enumerate() {
        let mut seen = HashSet::new();
        let mut neighbors = vec![*node];
        let mut dist = 0;
        while !neighbors.is_empty() {
            let mut new_neighbors = vec![];
            for neighbor in neighbors {
                if seen.contains(neighbor) { continue; }
                if let Some(j) = good_node.iter().position(|s| neighbor == *s) {
                    dist_matrix[i][j] = (dist, input.get(&neighbor.to_string()).unwrap().0);
                }
                seen.insert(neighbor);

                input.get(&neighbor.to_string()).unwrap().1.iter()
                    .for_each(|s| new_neighbors.push(s.as_str()))
            }
            neighbors = new_neighbors;
            dist += 1;
        }
    }

    let start_index = good_node.len() as u32 - 1;
    let mut all = 1;
    for _ in 1..good_node.len() {
        all = (all << 1) + 1;
    }

    let part1 = part_pressure(&dist_matrix, start_index, 1 << start_index, all, 30)
        .iter().fold(0, |acc, x| acc.max(x.0));
    
    let mut min_26 = part_pressure(&dist_matrix, start_index, 1 << start_index, all, 26)
        .iter().map(|x| *x).collect::<Vec<_>>();
    min_26.sort_by(|a, b| b.0.cmp(&a.0));
    let mut part2 = 0;
    for i in 0..min_26.len()-1 {
        for j in i+1..min_26.len() {
            if min_26[i].1 & min_26[j].1 == 1 << start_index {
                part2 = part2.max(min_26[i].0 + min_26[j].0);
            }
        }
    }

    return (part1, part2);
}