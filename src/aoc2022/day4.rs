use crate::AOCRunnable;
use anyhow::Result;
use std::num::ParseIntError;
use std::ops::RangeInclusive;

pub struct AOCDay;

fn get_range<S: Into<String>>(input: S) -> Option<RangeInclusive<i32>> {
    let nums: Result<Vec<i32>, ParseIntError> =
        input.into().split('-').map(|i| i.parse::<i32>()).collect();
    if nums.is_err() {
        return None;
    }
    let nums = nums.unwrap();

    let a = nums.first().unwrap_or(&0).to_owned();
    let b = nums.get(1).unwrap_or(&0).to_owned();

    Some(a..=b)
}

fn overlapping(mut a: RangeInclusive<i32>, mut b: RangeInclusive<i32>) -> bool {
    a.clone().all(|c| b.clone().any(|c1| c == c1)) || b.all(|c| a.any(|c1| c == c1))
}

fn overlapping_any(mut a: RangeInclusive<i32>, mut b: RangeInclusive<i32>) -> bool {
    a.clone().any(|c| b.clone().any(|c1| c == c1)) || b.any(|c| a.any(|c1| c == c1))
}

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> Result<String> {
        let pairs = input.split('\n').filter_map(|line| {
            let split = line.split(',');

            let ranges: Option<Vec<RangeInclusive<i32>>> = split.map(get_range).collect();
            let ranges = ranges?;

            let overlapping = overlapping(
                ranges.first().unwrap().to_owned(),
                ranges.get(1).unwrap().to_owned(),
            );

            if overlapping {
                Some(())
            } else {
                None
            }
        });

        Ok(pairs.count().to_string())
    }

    fn run_pt2(input: String) -> Result<String> {
        let pairs = input.split('\n').filter_map(|line| {
            let split = line.split(',');

            let ranges: Option<Vec<RangeInclusive<i32>>> = split.map(get_range).collect();
            let ranges = ranges?;

            let overlapping = overlapping_any(
                ranges.first().unwrap().to_owned(),
                ranges.get(1).unwrap().to_owned(),
            );

            if overlapping {
                Some(())
            } else {
                None
            }
        });

        Ok(pairs.count().to_string())
    }
}
