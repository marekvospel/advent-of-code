use advent_of_code::{get_input, Result};

use advent_of_code::AOCRunnable;

#[test]
fn run_day1_test() -> Result<()> {
    let input = get_input("day1-test.txt")?;
    let result = advent_of_code::day1::Day1::run_pt1(input.clone());
    println!("Result: {}", result);
    assert_eq!(result, "24000");
    let result = advent_of_code::day1::Day1::run_pt2(input);
    println!("Result: {}", result);
    assert_eq!(result, "45000");
    Ok(())
}

#[test]
fn run_day1() -> Result<()> {
    let input = get_input("day1.txt")?;
    let result = advent_of_code::day1::Day1::run_pt1(input.clone());
    println!("Result: {}", result);
    assert_eq!(result, "66719");
    let result = advent_of_code::day1::Day1::run_pt2(input);
    println!("Result: {}", result);
    assert_eq!(result, "198551");
    Ok(())
}
