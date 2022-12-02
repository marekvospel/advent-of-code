use crate::AOCRunnable;

pub struct AOCDay;

#[derive(Clone, Debug)]
enum Shape {
  Rock,
  Paper,
  Scissors,
  None,
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
      _ => Shape::None
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
    }
    Shape::Paper => match opponent.into() {
      Shape::Rock => 6,
      Shape::Paper => 3,
      Shape::Scissors => 0,
      Shape::None => 0,
    }
    Shape::Scissors => match opponent.into() {
      Shape::Rock => 0,
      Shape::Paper => 6,
      Shape::Scissors => 3,
      Shape::None => 0,
    }
    Shape::None => 0,
  }
}

impl AOCRunnable for AOCDay {
  fn run_pt1(input: String) -> String {
    // First in touple is me, second is opponent
    let matches: Vec<(Shape, Shape)> = input.split("\n").filter_map(|s| {
      let mut split = s.chars();
      if split.clone().collect::<Vec<char>>().len() < 2 {
        return None
      }

      Some((Shape::from(split.next().unwrap()), Shape::from(split.skip(1).next().unwrap())))
    }).collect();

    let score: i32 = matches.iter().map(|(me, opponent)| compare(me.clone(), opponent.clone())).sum();
    println!("Score: {}", score);
    let play_score = matches.iter().map(|(me, _)| me.to_i32()).sum::<i32>();
    println!("Play score: {}", play_score);

    let result = score + play_score;

    result.to_string()
  }

  fn run_pt2(input: String) -> String {
    todo!()
  }
}
