use std::fs::read_to_string;
use std::path::PathBuf;

pub mod day1;

pub trait AOCRunnable {
    fn run_pt1(input: String) -> String;
    fn run_pt2(input: String) -> String;
}

pub type Result<T> = std::result::Result<T, std::io::Error>;

pub fn get_input<S: Into<String>>(day: S) -> Result<String> {
    let mut day_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    day_path.push("tests/inputs");
    day_path.push(day.into());
    read_to_string(day_path)
}
