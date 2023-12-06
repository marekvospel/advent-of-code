use crate::AOCRunnable;

pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> anyhow::Result<String> {
        let mut input = input.split('\n');
        let mut line1 = input.next().unwrap().split_whitespace().skip(1);
        let mut line2 = input.next().unwrap().split_whitespace().skip(1);

        let mut races = vec![];

        for _ in 0..*&line1.clone().count() {
            let num = line1.next().unwrap();
            let num2 = line2.next().unwrap();

            races.push(Race::parse(num, num2));
        }

        let multi = races
            .iter()
            .map(|r| r.winning_configs())
            .reduce(|a, b| a * b)
            .unwrap_or(0);

        Ok(multi.to_string())
    }

    fn run_pt2(input: String) -> anyhow::Result<String> {
        let mut input = input.split('\n');
        let line1 = input
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .collect::<String>();
        let line2 = input
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .collect::<String>();

        let num = line1;
        let num2 = line2;

        let race = Race::parse(&num, &num2);

        Ok(race.winning_configs().to_string())
    }
}

#[derive(Debug)]
pub struct Race {
    duration: u64,
    record: u64,
}

impl Race {
    fn parse(line0: &str, line1: &str) -> Self {
        let duration = line0.parse().unwrap();
        let record = line1.parse().unwrap();
        Self { duration, record }
    }

    fn winning_configs(&self) -> u64 {
        let mut out = 0;

        for i in 0..self.duration {
            if self.record < (self.duration - i) * i {
                out += 1;
            }
        }

        out
    }
}
