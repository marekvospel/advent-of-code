use crate::AOCRunnable;

pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> anyhow::Result<String> {
        let lines = input
            .trim_end()
            .lines()
            .map(Values::parse)
            .map(Values::into_predict)
            .reduce(|a, b| b + a)
            .unwrap();

        Ok(lines.to_string())
    }

    fn run_pt2(input: String) -> anyhow::Result<String> {
        let lines = input
            .trim_end()
            .lines()
            .map(Values::parse)
            .map(Values::into_predict_pre)
            .reduce(|a, b| b + a)
            .unwrap();

        Ok(lines.to_string())
    }
}

#[derive(Debug)]
struct Values {
    values: Vec<i64>,
    next: Option<Box<Self>>,
}

impl Values {
    fn parse(input: &str) -> Self {
        let numbers: Vec<i64> = input
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        Self::create(numbers)
    }

    fn create(numbers: Vec<i64>) -> Self {
        let mut next_line = vec![];

        if numbers.iter().all(|i| *i == 0) {
            return Self {
                values: numbers,
                next: None,
            };
        }

        for i in 1..numbers.len() {
            next_line.push(numbers[i] - numbers[i - 1]);
        }

        let next = Self::create(next_line);

        Self {
            values: numbers,
            next: Some(Box::new(next)),
        }
    }

    fn into_predict(self) -> i64 {
        self.predict()
    }

    fn predict(&self) -> i64 {
        *self.values.last().unwrap()
            + match &self.next {
                Some(val) => val.predict(),
                None => 0,
            }
    }

    fn into_predict_pre(self) -> i64 {
        self.predict_pre()
    }

    fn predict_pre(&self) -> i64 {
        *self.values.first().unwrap()
            - match &self.next {
                Some(val) => val.predict_pre(),
                None => 0,
            }
    }
}
