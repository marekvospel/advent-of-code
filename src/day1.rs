use crate::AOCRunnable;

pub struct Day1 {}

impl AOCRunnable for Day1 {
    fn run_pt1(input: String) -> String {
        // Each index is inventory of one elf
        let elves_split: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();

        println!("Total elves: {}", elves_split.len());

        let mut elves_calories = Vec::new();

        for elf in elves_split {
            let mut elf_cals = 0;
            elf.split('\n').for_each(|i| {
                if let Ok(i) = i.parse::<i32>() {
                    elf_cals += i;
                }
            });
            if elf_cals == 0 {
                continue;
            }
            elves_calories.push(elf_cals);
        }

        elves_calories.sort();
        elves_calories.reverse();

        println!("Most calories: {}", elves_calories.first().unwrap_or(&0));
        println!("Least calories: {}", elves_calories.last().unwrap_or(&0));

        elves_calories.first().unwrap_or(&0).to_string()
    }

    fn run_pt2(input: String) -> String {
        // Each index is inventory of one elf
        let elves_split: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();

        println!("Total elves: {}", elves_split.len());

        let mut elves_calories = Vec::new();

        for elf in elves_split {
            let mut elf_cals = 0;
            elf.split('\n').for_each(|i| {
                if let Ok(i) = i.parse::<i32>() {
                    elf_cals += i;
                }
            });
            if elf_cals == 0 {
                continue;
            }
            elves_calories.push(elf_cals);
        }

        elves_calories.sort();
        elves_calories.reverse();

        println!("Most calories: {}", elves_calories.first().unwrap_or(&0));
        println!("Least calories: {}", elves_calories.last().unwrap_or(&0));

        elves_calories[0..=2].iter().sum::<i32>().to_string()
    }
}
