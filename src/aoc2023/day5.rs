use std::cmp::min;

use crate::AOCRunnable;
pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> anyhow::Result<String> {
        let mut input = input.split("\n\n");
        let mut numbers: Vec<u64> = input
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect();

        let maps: Vec<Map> = input.map(Map::parse).collect();

        for map in maps {
            let mut mapped = vec![];
            for number in numbers.iter() {
                mapped.push(map.map(*number));
            }
            numbers = mapped;
        }

        let mut out = u64::MAX;

        for number in numbers {
            out = min(out, number);
        }

        Ok(out.to_string())
    }

    // Part 2 is expensive to run, my answer is 104070862, but to save on test time, I didn't put it to the solution txt
    fn run_pt2(input: String) -> anyhow::Result<String> {
        let mut input = input.split("\n\n");
        let mut numbers: Vec<u64> = vec![];
        let mut seeds = input.next().unwrap().split_whitespace().skip(1);

        while let Some(seed) = seeds.next() {
            let start: u64 = seed.parse().unwrap();
            let len: u64 = seeds.next().unwrap().parse().unwrap();

            for i in 0..len {
                numbers.push(start + i);
            }
        }

        let maps: Vec<Map> = input.map(Map::parse).collect();

        for number in numbers.iter_mut() {
            for map in maps.iter() {
                *number = map.map(*number);
            }
        }

        let mut out = u64::MAX;

        for number in numbers {
            out = min(out, number);
        }

        Ok(out.to_string())
    }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut ranges = vec![];

        for range in input.split('\n').skip(1) {
            ranges.push(MapRange::parse(range));
        }

        Self { ranges }
    }

    fn map(&self, number: u64) -> u64 {
        let range = self
            .ranges
            .iter()
            .find(|r| r.source <= number && number < r.source + r.length);
        match range {
            Some(range) => range.target + (number - range.source),
            None => number,
        }
    }
}

#[derive(Debug)]
struct MapRange {
    source: u64,
    target: u64,
    length: u64,
}

impl MapRange {
    fn parse(input: &str) -> Self {
        let mut split = input.split_whitespace();
        let target = split.next().unwrap().parse().unwrap();
        let source = split.next().unwrap().parse().unwrap();
        let length = split.next().unwrap().parse().unwrap();

        Self {
            source,
            target,
            length,
        }
    }
}
