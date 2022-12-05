use advent_of_code::{get_input, Result};

use advent_of_code::day5::AOCDay;
use advent_of_code::AOCRunnable;

#[test]
fn run_day5_test() -> Result<()> {
    let input = get_input("day5-test.txt")?;
    let result = AOCDay::run_pt1(input.clone())?;
    println!("Result: {}", result);
    assert_eq!(result, "CMZ");
    let result = AOCDay::run_pt2(input)?;
    println!("Result: {}", result);
    assert_eq!(result, "MCD");
    Ok(())
}

#[test]
fn run_day5() -> Result<()> {
    let input = get_input("day5.txt")?;
    let result = AOCDay::run_pt1(input.clone())?;
    println!("Result: {}", result);
    assert_eq!(result, "VGBBJCRMN");
    let result = AOCDay::run_pt2(input)?;
    println!("Result: {}", result);
    assert_eq!(result, "LBBVJBRMH");
    Ok(())
}
