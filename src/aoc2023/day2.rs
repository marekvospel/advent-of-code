use std::cmp::max;

use crate::AOCRunnable;

pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> anyhow::Result<String> {
        let games = input
            .split('\n')
            .filter(|s| s != &"")
            .map(|s| Game::parse(s.to_string()))
            .collect::<Vec<Game>>();

        let valid = games.iter().filter_map(|g| {
            if g.hands
                .iter()
                .any(|h| h.red > 12 || h.green > 13 || h.blue > 14)
            {
                None
            } else {
                Some(g.id)
            }
        });

        let sum = valid.reduce(|a, b| a + b).unwrap();

        Ok(sum.to_string())
    }

    fn run_pt2(input: String) -> anyhow::Result<String> {
        let games = input
            .split('\n')
            .filter(|s| s != &"")
            .map(|s| Game::parse(s.to_string()))
            .collect::<Vec<Game>>();

        let most = games
            .iter()
            .map(|g| {
                g.hands.iter().fold((0, 0, 0), |a, b| {
                    (max(a.0, b.red), max(a.1, b.green), max(a.2, b.blue))
                })
            })
            .fold(0, |a, b| a + (b.0 * b.1 * b.2));

        Ok(most.to_string())
    }
}

struct Game {
    id: usize,
    hands: Vec<Hand>,
}

struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}

impl Game {
    fn parse(input: String) -> Self {
        let mut chars = input.split(": ");
        let game_id = chars
            .next()
            .unwrap()
            .chars()
            .skip(5)
            .collect::<String>()
            .parse::<usize>();

        let mut hands = vec![];

        for hand in chars.next().unwrap().split("; ") {
            hands.push(Hand::parse(hand.to_string()));
        }

        Self {
            id: game_id.unwrap(),
            hands,
        }
    }
}

impl Hand {
    fn parse(input: String) -> Self {
        let chars = input.split(", ");
        let mut colors = (0, 0, 0);

        for color in chars {
            let mut split = color.split_whitespace();
            let num = split.next().unwrap().parse::<usize>().unwrap();
            let color = split.next().unwrap();

            match color {
                "red" => colors.0 += num,
                "green" => colors.1 += num,
                "blue" => colors.2 += num,
                _ => panic!("invalid color"),
            }
        }

        Hand {
            red: colors.0,
            green: colors.1,
            blue: colors.2,
        }
    }
}
