use std::collections::HashMap;

use crate::AOCRunnable;

pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> anyhow::Result<String> {
        let mut input = input.trim_end().split("\n\n");

        let instructions = input.next().unwrap().chars().collect::<Vec<char>>();

        let lines = input.next().unwrap().split("\n");

        let mut nodes = HashMap::new();

        for line in lines {
            let mut separate = line.split_whitespace();
            let name = separate.next().unwrap();
            separate.next();
            let left = separate
                .next()
                .unwrap()
                .trim_matches('(')
                .trim_matches(',')
                .to_owned();
            let right = separate.next().unwrap().trim_matches(')').to_owned();

            let node = MapNode { left, right };
            nodes.insert(name, node);
        }

        let mut current_node = "AAA";

        let mut index = 0;
        loop {
            for instruction in (&instructions).iter() {
                index += 1;
                match instruction {
                    'R' => {
                        current_node = &nodes.get(current_node).unwrap().right;
                    }
                    'L' => {
                        current_node = &nodes.get(current_node).unwrap().left;
                    }
                    _ => {}
                }

                if current_node == "ZZZ" {
                    return Ok((index).to_string());
                }
            }
        }
    }

    fn run_pt2(input: String) -> anyhow::Result<String> {
        let mut input = input.trim_end().split("\n\n");

        let instructions = input.next().unwrap().chars().collect::<Vec<char>>();

        let lines = input.next().unwrap().split("\n");

        let mut nodes = HashMap::new();

        for line in lines {
            let mut separate = line.split_whitespace();
            let name = separate.next().unwrap();
            separate.next();
            let left = separate
                .next()
                .unwrap()
                .trim_matches('(')
                .trim_matches(',')
                .to_owned();
            let right = separate.next().unwrap().trim_matches(')').to_owned();

            let node = MapNode { left, right };
            nodes.insert(name, node);
        }

        let mut current_nodes = vec![];
        let mut ended = HashMap::new();

        for (key, _) in nodes.iter() {
            if key.ends_with('A') {
                current_nodes.push(*key)
            }
        }
        let len = current_nodes.len();

        let mut index = 0_u64;
        loop {
            for instruction in (&instructions).iter() {
                index += 1;
                for current_node in current_nodes.iter_mut() {
                    match instruction {
                        'R' => {
                            *current_node = &nodes.get(current_node).unwrap().right;
                        }
                        'L' => {
                            *current_node = &nodes.get(current_node).unwrap().left;
                        }
                        _ => {}
                    }

                    if current_node.ends_with("Z") {
                        if !&ended.contains_key(*current_node) {
                            ended.insert(*current_node, index);
                            if ended.len() == len {
                                return Ok((lcm_of_list(
                                    ended.into_values().collect::<Vec<u64>>().as_slice(),
                                ))
                                .to_string());
                            }
                        }
                    }
                }
            }
        }
    }
}

struct MapNode {
    left: String,
    right: String,
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn lcm_of_list(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .copied()
        .reduce(|a, b| lcm(a, b))
        .unwrap_or(1)
}
