use crate::{AOCResult, AOCRunnable};

pub struct AOCDay;

#[derive(Clone, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
    None,
}

#[derive(Clone, Debug)]
enum GameResult {
    Lose(Shape),
    Draw(Shape),
    Win(Shape),
    None,
}

impl GameResult {
    fn from_result(value: char, opponent: Shape) -> Self {
        match value {
            'X' => GameResult::Lose(opponent),
            'Y' => GameResult::Draw(opponent),
            'Z' => GameResult::Win(opponent),
            _ => GameResult::None,
        }
    }
}

impl From<char> for Shape {
    fn from(value: char) -> Self {
        match value {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,

            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => Shape::None,
        }
    }
}

impl From<GameResult> for Shape {
    fn from(value: GameResult) -> Self {
        match value {
            GameResult::Win(shape) => match shape {
                Shape::Rock => Shape::Paper,
                Shape::Scissors => Shape::Rock,
                Shape::Paper => Shape::Scissors,
                Shape::None => Shape::None,
            },
            GameResult::Draw(shape) => match shape {
                Shape::Rock => Shape::Rock,
                Shape::Scissors => Shape::Scissors,
                Shape::Paper => Shape::Paper,
                Shape::None => Shape::None,
            },
            GameResult::Lose(shape) => match shape {
                Shape::Rock => Shape::Scissors,
                Shape::Scissors => Shape::Paper,
                Shape::Paper => Shape::Rock,
                Shape::None => Shape::None,
            },
            GameResult::None => Shape::None,
        }
    }
}

impl Shape {
    fn to_i32(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
            Shape::None => 0,
        }
    }
}

fn compare<S: Into<Shape>>(my: S, opponent: S) -> i32 {
    match my.into() {
        Shape::Rock => match opponent.into() {
            Shape::Rock => 3,
            Shape::Paper => 0,
            Shape::Scissors => 6,
            Shape::None => 0,
        },
        Shape::Paper => match opponent.into() {
            Shape::Rock => 6,
            Shape::Paper => 3,
            Shape::Scissors => 0,
            Shape::None => 0,
        },
        Shape::Scissors => match opponent.into() {
            Shape::Rock => 0,
            Shape::Paper => 6,
            Shape::Scissors => 3,
            Shape::None => 0,
        },
        Shape::None => 0,
    }
}

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> AOCResult<String> {
        // First in tuple is opponent, second is me
        let matches: Vec<(Shape, Shape)> = input
            .split('\n')
            .filter_map(|s| {
                let mut split = s.chars();
                if split.clone().collect::<Vec<char>>().len() < 2 {
                    return None;
                }

                Some((
                    Shape::from(split.next().unwrap()),
                    Shape::from(split.nth(1).unwrap()),
                ))
            })
            .collect();

        let score: i32 = matches
            .iter()
            .map(|(opponent, me)| compare(me.clone(), opponent.clone()))
            .sum();
        println!("Score: {}", score);
        let play_score = matches.iter().map(|(_, me)| me.to_i32()).sum::<i32>();
        println!("Play score: {}", play_score);

        let result = score + play_score;

        Ok(result.to_string())
    }

    fn run_pt2(input: String) -> AOCResult<String> {
        // First in tuple is opponent, second is me
        let matches: Vec<(Shape, Shape)> = input
            .split('\n')
            .filter_map(|s| {
                let mut split = s.chars();
                if split.clone().collect::<Vec<char>>().len() < 2 {
                    return None;
                }

                let opponent = Shape::from(split.next().unwrap());

                Some((
                    opponent.clone(),
                    Shape::from(GameResult::from_result(split.nth(1).unwrap(), opponent)),
                ))
            })
            .collect();

        let score: i32 = matches
            .iter()
            .map(|(opponent, me)| compare(me.clone(), opponent.clone()))
            .sum();
        println!("Score: {}", score);
        let play_score = matches.iter().map(|(_, me)| me.to_i32()).sum::<i32>();
        println!("Play score: {}", play_score);

        let result = score + play_score;

        Ok(result.to_string())
    }
}
