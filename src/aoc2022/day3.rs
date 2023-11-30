use crate::AOCRunnable;
use anyhow::Result;

pub struct AOCDay;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Bag {
    compartment1: String,
    compartment2: String,
}

impl Bag {
    fn new<S: Into<String>>(line: S) -> Self {
        let line = line.into();
        let compartment1 = line.chars().take(line.len() / 2).collect::<String>();
        let compartment2 = line.chars().skip(line.len() / 2).collect::<String>();
        Bag {
            compartment1,
            compartment2,
        }
    }

    fn get_common(&self) -> i32 {
        for c in self.compartment1.chars() {
            for c2 in self.compartment2.chars() {
                if c == c2 {
                    return char_value(c);
                }
            }
        }

        0
    }
}

fn common_in_bags(bags: &[Bag]) -> i32 {
    let bag = bags.first();
    if bag.is_none() {
        return 0;
    }
    let bag = bag.unwrap();

    let bag_contents = bag.compartment1.clone() + &*bag.compartment2.clone();

    for c in bag_contents.chars() {
        if bags.iter().all(|bag| {
            let bag_contents = bag.compartment1.clone() + &*bag.compartment2.clone();
            bag_contents.chars().any(|c1| c == c1)
        }) {
            return char_value(c);
        }
    }

    0
}

fn char_value(c: char) -> i32 {
    (match c {
        'a'..='z' => (c as u32) - 96,
        'A'..='Z' => (c as u32) - 38,
        _ => 0,
    } as i32)
}

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> Result<String> {
        let bags = input.split('\n').map(Bag::new).collect::<Vec<Bag>>();

        let result: i32 = bags.iter().map(|bag| bag.get_common()).sum();

        Ok(result.to_string())
    }

    fn run_pt2(input: String) -> Result<String> {
        let bags = input.split('\n').map(Bag::new).collect::<Vec<Bag>>();

        let mut groups = Vec::new();
        let mut curr_group = Vec::new();

        for bag in bags {
            curr_group.push(bag);

            if curr_group.len() > 2 {
                groups.push(curr_group);
                curr_group = Vec::new();
            }
        }
        let result: i32 = groups.iter().map(|bags| common_in_bags(bags)).sum();

        Ok(result.to_string())
    }
}
