use advent_of_code::{get_input, Result};

use advent_of_code::AOCRunnable;
use advent_of_code::day2::AOCDay;

#[test]
fn run_day2_test() -> Result<()> {
    let input = get_input("day2-test.txt")?;
    let result = AOCDay::run_pt1(input.clone());
    println!("Result: {}", result);
    assert_eq!(result, "15");
    return Ok(());
    let result = AOCDay::run_pt2(input);
    println!("Result: {}", result);
    assert_eq!(result, "45000");
    Ok(())
}

#[test]
fn run_day2() -> Result<()> {
    let input = get_input("day2.txt")?;
    let result = AOCDay::run_pt1(input.clone());
    println!("Result: {}", result);
    assert_eq!(result, "");
    return Ok(());
    let result = AOCDay::run_pt2(input);
    println!("Result: {}", result);
    assert_eq!(result, "");
    Ok(())
}
