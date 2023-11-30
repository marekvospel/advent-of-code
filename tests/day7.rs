use advent_of_code::aoc2022::day7::AOCDay;
use advent_of_code::get_input;
use anyhow::Result;

use advent_of_code::AOCRunnable;

#[test]
fn run_day7_test() -> Result<()> {
    let input = get_input("day7-test.txt")?;
    let result = AOCDay::run_pt1(input.clone())?;
    println!("Result: {}", result);
    assert_eq!(result, "95437");
    let result = AOCDay::run_pt2(input)?;
    println!("Result: {}", result);
    assert_eq!(result, "24933642");
    Ok(())
}

#[test]
fn run_day7() -> Result<()> {
    let input = get_input("day7.txt")?;
    let result = AOCDay::run_pt1(input.clone())?;
    println!("Result: {}", result);
    assert_eq!(result, "2104783");
    let result = AOCDay::run_pt2(input)?;
    println!("Result: {}", result);
    assert_eq!(result, "5883165");
    Ok(())
}
