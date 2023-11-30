use advent_of_code::aoc2022::day4::AOCDay;
use advent_of_code::get_input;
use anyhow::Result;

use advent_of_code::AOCRunnable;

#[test]
fn run_day4_test() -> Result<()> {
    let input = get_input("day4-test.txt")?;
    let result = AOCDay::run_pt1(input.clone())?;
    println!("Result: {}", result);
    assert_eq!(result, "2");
    let result = AOCDay::run_pt2(input)?;
    println!("Result: {}", result);
    assert_eq!(result, "4");
    Ok(())
}

#[test]
fn run_day4() -> Result<()> {
    let input = get_input("day4.txt")?;
    let result = AOCDay::run_pt1(input.clone())?;
    println!("Result: {}", result);
    assert_eq!(result, "498");
    let result = AOCDay::run_pt2(input)?;
    println!("Result: {}", result);
    assert_eq!(result, "859");
    Ok(())
}
