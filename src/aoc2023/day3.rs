use std::{cmp::min, collections::HashSet};

use crate::AOCRunnable;

pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> anyhow::Result<String> {
        let engine = Engine::parse(input);
        let mut sum = 0;

        for number in engine.numbers {
            let row_start = if number.row == 0 { 0 } else { number.row - 1 };
            let row_end = min(engine.rows.len() - 1, number.row + 1);
            let col_start = if number.start == 0 {
                0
            } else {
                number.start - 1
            };
            let col_end = min(engine.width - 1, number.end + 1);

            if engine.rows[row_start..=row_end]
                .iter()
                .any(|r| r[col_start..=col_end].iter().any(|c| c.is_symbol()))
            {
                sum += number.number;
            }
        }

        Ok(sum.to_string())
    }

    fn run_pt2(input: String) -> anyhow::Result<String> {
        let engine = Engine::parse(input);
        let mut sum = 0;

        for (row_i, row) in engine.rows.iter().enumerate() {
            for (col_i, col) in row.iter().enumerate() {
                if matches!(col, EngineChar::Gear) {
                    let row_start = if row_i == 0 { 0 } else { row_i - 1 };
                    let row_end = min(engine.rows.len() - 1, row_i + 1);
                    let col_start = if col_i == 0 { 0 } else { col_i - 1 };
                    let col_end = min(engine.width - 1, col_i + 1);
                    let mut nums = HashSet::new();

                    for row in engine.rows[row_start..=row_end].iter() {
                        for col in row[col_start..=col_end].iter() {
                            if let EngineChar::Number(n) = col {
                                nums.insert(*n);
                            }
                        }
                    }
                    let mut nums = nums.into_iter();

                    let first = nums.next();
                    let second = nums.next();

                    if first.is_some() && second.is_some() {
                        sum += engine.numbers[first.unwrap()].number
                            * engine.numbers[second.unwrap()].number;
                    }
                }
            }
        }

        Ok(sum.to_string())
    }
}

#[derive(Debug)]
struct Engine {
    rows: Vec<Vec<EngineChar>>,
    numbers: Vec<EngineNumber>,
    width: usize,
}

#[derive(Debug)]
enum EngineChar {
    Empty,
    Number(usize),
    Gear,
    OtherSymbol,
}

impl EngineChar {
    fn is_symbol(&self) -> bool {
        match self {
            Self::Empty => false,
            Self::Number(_) => false,
            Self::Gear => true,
            Self::OtherSymbol => true,
        }
    }
}

#[derive(Debug)]
struct EngineNumber {
    number: usize,
    row: usize,
    /// Column start
    start: usize,
    /// Column end
    end: usize,
}

impl Engine {
    pub fn parse(input: String) -> Self {
        let input_rows = input.split('\n');
        let mut rows = vec![];
        let mut numbers = vec![];
        let mut width = 0;

        let mut current_number = String::new();
        let mut number_index = 0;

        fn save_number(
            numbers: &mut Vec<EngineNumber>,
            current_number: &mut String,
            number_index: &mut usize,
            row: usize,
        ) {
            numbers.push(EngineNumber {
                number: current_number.parse().unwrap(),
                row: row,
                start: *number_index,
                end: *number_index + current_number.len() - 1,
            });
            current_number.clear();
            *number_index = 0;
        }

        for (row_i, row) in input_rows.filter(|s| s != &"").enumerate() {
            if row_i == 0 {
                width = row.len();
            }
            let mut row_vec = vec![];

            for (col, c) in row.chars().enumerate() {
                row_vec.push(match c {
                    '0'..='9' => {
                        if current_number.is_empty() {
                            number_index = col;
                        }
                        current_number.push(c);
                        EngineChar::Number(numbers.len())
                    }
                    _ => {
                        if !current_number.is_empty() {
                            save_number(
                                &mut numbers,
                                &mut current_number,
                                &mut number_index,
                                row_i,
                            );
                        }
                        match c {
                            '*' => EngineChar::Gear,
                            '.' => EngineChar::Empty,
                            _ => EngineChar::OtherSymbol,
                        }
                    }
                })
            }

            if !current_number.is_empty() {
                save_number(&mut numbers, &mut current_number, &mut number_index, row_i);
            }
            rows.push(row_vec);
        }

        Self {
            rows,
            numbers,
            width,
        }
    }
}
