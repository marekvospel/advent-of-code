use std::fs::read_to_string;
use std::path::PathBuf;

extern crate anyhow;

pub mod aoc2022;

pub trait AOCRunnable {
    fn run_pt1(input: String) -> anyhow::Result<String>;
    fn run_pt2(input: String) -> anyhow::Result<String>;
}

pub fn get_input<S: Into<String>>(year: S, day: S) -> anyhow::Result<String> {
    let mut day_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    day_path.push(format!("tests/fixtures/{}/input", year.into()));
    day_path.push(day.into());
    Ok(read_to_string(day_path)?)
}

pub fn get_solution<S: Into<String>>(year: S, day: S) -> anyhow::Result<(String, Option<String>)> {
    let mut day_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    day_path.push(format!("tests/fixtures/{}/solution", year.into()));
    day_path.push(day.into());
    let split = read_to_string(day_path)?;
    let mut split = split.split('\n').into_iter();
    let one = split.next().map(|s| s.to_string());
    let two = split.next().map(|s| s.to_string());
    Ok((one.unwrap(), two))
}
