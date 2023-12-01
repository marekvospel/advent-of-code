use crate::AOCRunnable;

pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> anyhow::Result<String> {
        let lines = input.split('\n');
        let mut sum = 0;

        for line in lines {
            if line == "" {
                continue;
            }

            let first = line
                .chars()
                .find(|c| match c {
                    '0'..='9' => true,
                    _ => false,
                })
                .unwrap();
            let last = line
                .chars()
                .rev()
                .find(|c| match c {
                    '0'..='9' => true,
                    _ => false,
                })
                .unwrap();
            let combined = format!("{}{}", first, last);
            let num = combined.parse::<i32>().unwrap();
            sum += num;
        }

        Ok(sum.to_string())
    }

    fn run_pt2(input: String) -> anyhow::Result<String> {
        let lines = input.split('\n');
        let mut sum = 0;

        for line in lines {
            if line == "" {
                continue;
            }

            let mut first = None;
            let mut last = None;

            for i in 0..line.len() {
                let current = line.chars().skip(i).take(5).collect::<String>();
                let mut current = current.to_string();
                if current.len() < 5 {
                    current = format!("{current}{}", "\0".repeat(5 - current.len()));
                }

                mapper(&mut first, &current, false);
            }
            for i in 0..line.len() {
                let current = line.chars().rev().skip(i).take(5).collect::<String>();
                let mut current = current.to_string();
                if current.len() < 5 {
                    current = format!("{current}{}", "\0".repeat(5 - current.len()));
                }
                mapper(&mut last, &current, true);
            }

            let combined = format!("{}{}", first.unwrap(), last.unwrap());
            let num = combined.parse::<i32>().unwrap();
            sum += num;
        }

        Ok(sum.to_string())
    }
}

// To be honest I'm kinda dissapointed I didn't find a better solution
fn mapper(out: &mut Option<String>, current: &str, rev: bool) {
    if out.is_some() {
        return;
    }

    if rev {
        if &current[0..3] == "eno" {
            *out = Some("1".to_string());
            return;
        } else if &current[0..3] == "owt" {
            *out = Some("2".to_string());
            return;
        } else if &current[0..5] == "eerht" {
            *out = Some("3".to_string());
            return;
        } else if &current[0..4] == "ruof" {
            *out = Some("4".to_string());
            return;
        } else if &current[0..4] == "evif" {
            *out = Some("5".to_string());
            return;
        } else if &current[0..3] == "xis" {
            *out = Some("6".to_string())
        } else if &current[0..5] == "neves" {
            *out = Some("7".to_string());
            return;
        } else if &current[0..5] == "thgie" {
            *out = Some("8".to_string());
            return;
        } else if &current[0..4] == "enin" {
            *out = Some("9".to_string());
            return;
        }
    } else {
        if &current[0..3] == "one" {
            *out = Some("1".to_string());
            return;
        } else if &current[0..3] == "two" {
            *out = Some("2".to_string());
            return;
        } else if &current[0..5] == "three" {
            *out = Some("3".to_string());
            return;
        } else if &current[0..4] == "four" {
            *out = Some("4".to_string());
            return;
        } else if &current[0..4] == "five" {
            *out = Some("5".to_string());
            return;
        } else if &current[0..3] == "six" {
            *out = Some("6".to_string())
        } else if &current[0..5] == "seven" {
            *out = Some("7".to_string());
            return;
        } else if &current[0..5] == "eight" {
            *out = Some("8".to_string());
            return;
        } else if &current[0..4] == "nine" {
            *out = Some("9".to_string());
            return;
        }
    }
    let first = current.chars().next().unwrap();
    match first {
        '0'..='9' => {
            *out = Some(first.to_string());
            return;
        }
        _ => {}
    }
}
