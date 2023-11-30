use crate::AOCRunnable;
use anyhow::Result;

pub struct AOCDay {}

fn run(input: String) -> Result<Vec<i32>> {
    // Each index is inventory of one elf
    let elves_split: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();

    println!("Total elves: {}", elves_split.len());

    let mut elves_calories = Vec::new();

    for elf in elves_split {
        let mut elf_cals = 0;
        let _ = elf.split('\n').try_for_each(|i| {
            elf_cals += i.parse::<i32>()?;
            Ok::<(), anyhow::Error>(())
        });
        if elf_cals == 0 {
            continue;
        }
        elves_calories.push(elf_cals);
    }

    elves_calories.sort();
    elves_calories.reverse();
    Ok(elves_calories)
}

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> Result<String> {
        let elves_calories = run(input)?;

        println!("Most calories: {}", elves_calories.first().unwrap_or(&0));
        println!("Least calories: {}", elves_calories.last().unwrap_or(&0));

        Ok(elves_calories.first().unwrap_or(&0).to_string())
    }

    fn run_pt2(input: String) -> Result<String> {
        let elves_calories = run(input)?;

        println!("Most calories: {}", elves_calories.first().unwrap_or(&0));
        println!("Least calories: {}", elves_calories.last().unwrap_or(&0));

        Ok(elves_calories[0..=2].iter().sum::<i32>().to_string())
    }
}
