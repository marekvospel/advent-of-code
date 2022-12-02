use advent_of_code::{get_input, Result};

#[test]
fn get_input_should_get_test_input() -> Result<()> {
    let input = get_input("test.txt")?;
    assert_eq!(input, "123456789\n".to_string());
    Ok(())
}

// mod day1;
