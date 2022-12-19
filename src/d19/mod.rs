use std::fs;

struct Blueprint {
    ore_robot_cost: i32,
    clay_robot_cost: i32,
    obsidian_robot_cost: (i32, i32),
    geode_robot_cost: (i32, i32)
}

impl Blueprint {

    fn get_part_geode(&self, time: i32, robots: &[i32; 4], materials: &[i32; 4], highest: i32) -> i32 {
        let potential_highest = materials[3] + (robots[3] + time) * (robots[3] + time + 1) / 2;
        if time == 0 || potential_highest < highest { 
            return materials[3];
        }

        let max_ore = self.ore_robot_cost.max(self.clay_robot_cost).max(self.obsidian_robot_cost.0).max(self.geode_robot_cost.0);
        let max_clay = self.obsidian_robot_cost.1;
        let max_obsidian = self.geode_robot_cost.1;

        let mut new_materials = [robots[0] + materials[0], robots[1] + materials[1], robots[2] + materials[2], robots[3] + materials[3]];
        let mut new_robots = robots.clone();
        let mut max_geode = highest;
        if materials[0] >= self.geode_robot_cost.0 && materials[2] >= self.geode_robot_cost.1 {
            new_materials[0] -= self.geode_robot_cost.0;
            new_materials[2] -= self.geode_robot_cost.1;
            new_robots[3] += 1;
            max_geode = max_geode.max(self.get_part_geode(time - 1, &new_robots, &new_materials, max_geode));
            new_materials[0] += self.geode_robot_cost.0;
            new_materials[2] += self.geode_robot_cost.1;
            new_robots[3] -= 1;
        } else {
            if robots[2] < max_obsidian && materials[0] >= self.obsidian_robot_cost.0 && materials[1] >= self.obsidian_robot_cost.1 {
                new_materials[0] -= self.obsidian_robot_cost.0;
                new_materials[1] -= self.obsidian_robot_cost.1;
                new_robots[2] += 1;
                max_geode = max_geode.max(self.get_part_geode(time - 1, &new_robots, &new_materials, max_geode));
                new_robots[2] -= 1;
                new_materials[1] += self.obsidian_robot_cost.1;
                new_materials[0] += self.obsidian_robot_cost.0;
            }
            if robots[1] < max_clay && materials[0] >= self.clay_robot_cost {
                new_materials[0] -= self.clay_robot_cost;
                new_robots[1] += 1;
                max_geode = max_geode.max(self.get_part_geode(time - 1, &new_robots, &new_materials, max_geode));
                new_robots[1] -= 1;
                new_materials[0] += self.clay_robot_cost;
            }
            if robots[0] < max_ore && materials[0] >= self.ore_robot_cost {
                new_materials[0] -= self.ore_robot_cost;
                new_robots[0] += 1;
                max_geode = max_geode.max(self.get_part_geode(time - 1, &new_robots, &new_materials, max_geode));
                new_robots[0] -= 1;
                new_materials[0] += self.ore_robot_cost;
            }
            max_geode = max_geode.max(self.get_part_geode(time - 1, robots, &new_materials, max_geode));
        }
        return max_geode;
    }

    fn get_most_geode(&self, time: i32) -> i32 {
        let robots = [1, 0, 0, 0];
        let materials = [0, 0, 0, 0];
        return self.get_part_geode(time, &robots, &materials, 0);
    }
}

fn get_input() -> Vec<Blueprint> {
    return fs::read_to_string("res/d19.txt").unwrap().lines()
        .map(|l| {
            match l.split_whitespace().skip(2).collect::<Vec<_>>().as_slice() {
                ["Each", "ore", "robot", "costs", a, "ore.", "Each", "clay", "robot", "costs", b, "ore.", "Each", "obsidian", "robot", "costs", c, "ore", "and", d, "clay.", "Each", "geode", "robot", "costs", e, "ore", "and", f, "obsidian."] => 
                    Blueprint { ore_robot_cost: a.parse().unwrap(), clay_robot_cost: b.parse().unwrap(), obsidian_robot_cost: (c.parse().unwrap(), d.parse().unwrap()), geode_robot_cost: (e.parse().unwrap(), f.parse().unwrap()) },
                _ => panic!()
            }
        })
        .collect();
}

pub fn get_quality_levels() -> i32 {
    let input = get_input();
    return input.iter().enumerate().map(|(i, b)| {
        let nr = i as i32 + 1;
        let geodes = b.get_most_geode(24);
        nr * geodes
    })
    .sum();
}

pub fn get_highest_geodes() -> i32 {
    return get_input().iter().take(3)
        .map(|b| b.get_most_geode(32))
        .fold(1, |x, acc| acc * x);
}