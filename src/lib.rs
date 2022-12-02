use std::fs::read_to_string;
use std::path::PathBuf;
use thiserror::Error;

pub mod day1;
pub mod day2;

pub trait AOCRunnable {
    fn run_pt1(input: String) -> AOCResult<String>;
    fn run_pt2(input: String) -> AOCResult<String>;
}

#[derive(Error, Debug)]
pub enum AOCError {
    #[error("There was an IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("There was an error while parsing an integer: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("There was an error while parsing challenge input")]
    ParseError(String),
}

pub type Result<T> = std::result::Result<T, AOCError>;
pub type AOCResult<T> = Result<T>;

pub fn get_input<S: Into<String>>(day: S) -> std::result::Result<String, AOCError> {
    let mut day_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    day_path.push("tests/fixtures");
    day_path.push(day.into());
    Ok(read_to_string(day_path)?)
}
