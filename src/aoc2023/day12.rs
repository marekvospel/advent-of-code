use crate::AOCRunnable;

pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> anyhow::Result<String> {
        let records: Vec<Record> = input.trim_end().lines().map(Record::parse).collect();

        Ok(records
            .iter()
            .fold(0, |a, b| a + b.combinations())
            .to_string())
    }

    fn run_pt2(input: String) -> anyhow::Result<String> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringState {
    Running,
    Broken,
    Unknown,
}

#[derive(Debug)]
struct Record {
    states: Vec<SpringState>,
    order: Vec<usize>,
}

impl Record {
    fn parse(input: &str) -> Self {
        let mut input = input.split_whitespace();
        let states = input
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                '.' => SpringState::Running,
                '#' => SpringState::Broken,
                _ => SpringState::Unknown,
            })
            .collect();

        let order = input
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        Self { states, order }
    }

    fn combinations(&self) -> u64 {
        combinations(self.states.clone(), &self.order)
    }
}

fn combinations(states: Vec<SpringState>, order: &[usize]) -> u64 {
    let mut decision = 0;
    // println!("{:?} {:?}", states, order);
    let last_unknown = states.len()
        - 1
        - states
            .iter()
            .rev()
            .position(|&o| o == SpringState::Unknown)
            .unwrap_or(0);

    for (i, &state) in states.iter().enumerate() {
        if state == SpringState::Unknown {
            let mut new = states.clone();
            new[i] = SpringState::Broken;
            if valid(&new, &order) {
                decision += if i == last_unknown {
                    1
                } else {
                    combinations(new, &order)
                };
            }

            let mut new = states.clone();
            new[i] = SpringState::Running;
            if valid(&new, &order) {
                decision += if i == last_unknown {
                    1
                } else {
                    combinations(new, &order)
                };
            }
            break;
        }
    }

    decision
}

fn valid(states: &[SpringState], order: &[usize]) -> bool {
    let mut order_i = 0;
    let mut current_array = 0;

    for state in states {
        match state {
            SpringState::Running => {
                let current = match order.get(order_i) {
                    Some(s) => s,
                    None => continue,
                };

                if &current_array != current && current_array != 0 {
                    return false;
                }

                if current_array != 0 {
                    order_i += 1;
                }
                current_array = 0;
            }
            SpringState::Broken => {
                let current = match order.get(order_i) {
                    Some(s) => s,
                    None => return false,
                };
                current_array += 1;

                if current_array > *current {
                    return false;
                }
            }
            _ => return true,
        }
    }

    match order.get(order_i) {
        Some(s) => current_array == *s && order_i == order.len() - 1,
        None => order_i == order.len(),
    }
}

#[test]
fn test() {
    use SpringState::*;
    let states = [
        Running, Broken, Broken, Broken, Running, Broken, Broken, Running, Running, Broken,
        Running, Unknown,
    ];
    let ord = [3, 2, 1];

    assert!(valid(&states, &ord));
}
