use advent_of_code::aoc2022::day6::AOCDay;
use advent_of_code::get_input;
use anyhow::Result;

use advent_of_code::AOCRunnable;

#[test]
fn run_day6_test() -> Result<()> {
    let input = get_input("day6-test.txt")?;
    let result = AOCDay::run_pt1(input.clone())?;
    println!("Result: {}", result);
    assert_eq!(result, "7");
    let result = AOCDay::run_pt2(input)?;
    println!("Result: {}", result);
    assert_eq!(result, "19");
    Ok(())
}

#[test]
fn run_day6() -> Result<()> {
    let input = get_input("day6.txt")?;
    let result = AOCDay::run_pt1(input.clone())?;
    println!("Result: {}", result);
    assert_eq!(result, "1850");
    let result = AOCDay::run_pt2(input)?;
    println!("Result: {}", result);
    assert_eq!(result, "2823");
    Ok(())
}
