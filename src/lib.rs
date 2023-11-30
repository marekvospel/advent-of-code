use std::fs::read_to_string;
use std::path::PathBuf;

extern crate anyhow;

pub mod aoc2022;

pub trait AOCRunnable {
    fn run_pt1(input: String) -> anyhow::Result<String>;
    fn run_pt2(input: String) -> anyhow::Result<String>;
}

pub fn get_input<S: Into<String>>(day: S) -> anyhow::Result<String> {
    let mut day_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    day_path.push("tests/fixtures");
    day_path.push(day.into());
    Ok(read_to_string(day_path)?)
}
