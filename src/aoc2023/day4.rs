use std::collections::HashMap;

use crate::AOCRunnable;

pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> anyhow::Result<String> {
        let cards = input
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(Card::parse)
            .collect::<Vec<_>>();

        let num = cards.iter().map(|c| {
            let winning_numbers = c
                .numbers
                .iter()
                .filter(|number| c.winning.contains(*number))
                .count();

            if winning_numbers > 0 {
                2usize.pow((winning_numbers - 1) as u32)
            } else {
                0
            }
        });

        let num = num.reduce(|a, b| a + b).unwrap();

        Ok(num.to_string())
    }

    fn run_pt2(input: String) -> anyhow::Result<String> {
        let mut cards = input
            .split('\n')
            .filter(|l| !l.is_empty())
            .enumerate()
            .map(|(i, l)| (i, (Card::parse(l), 1usize)))
            .collect::<HashMap<usize, _>>();

        for index in 0..cards.len() {
            let card = cards.get(&index).unwrap();
            let count = card.1;

            let winning_numbers = card
                .0
                .numbers
                .iter()
                .filter(|number| card.0.winning.contains(*number))
                .count();

            for number in index + 1..index + winning_numbers + 1 {
                let card = cards.get_mut(&number).unwrap();
                card.1 += count;
            }
        }

        let num = cards.into_iter().fold(0, |a, b| a + b.1 .1);

        Ok(num.to_string())
    }
}

#[derive(Debug)]
struct Card {
    numbers: Vec<usize>,
    winning: Vec<usize>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let line = line.split(": ").skip(1).next().unwrap();
        let mut split = line.split("| ");

        let winning = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let numbers = split
            .next()
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Self { numbers, winning }
    }
}
