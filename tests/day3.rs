use advent_of_code::{get_input, Result};

use advent_of_code::day3::AOCDay;
use advent_of_code::AOCRunnable;

#[test]
fn run_day3_test() -> Result<()> {
    let input = get_input("day3-test.txt")?;
    let result = AOCDay::run_pt1(input.clone())?;
    println!("Result: {}", result);
    assert_eq!(result, "157");
    let result = AOCDay::run_pt2(input)?;
    println!("Result: {}", result);
    assert_eq!(result, "70");
    Ok(())
}

#[test]
fn run_day3() -> Result<()> {
    let input = get_input("day3.txt")?;
    let result = AOCDay::run_pt1(input.clone())?;
    println!("Result: {}", result);
    assert_eq!(result, "8493");
    let result = AOCDay::run_pt2(input)?;
    println!("Result: {}", result);
    assert_eq!(result, "2552");
    Ok(())
}
