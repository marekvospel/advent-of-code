use crate::{AOCResult, AOCRunnable};

pub struct AOCDay;

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> AOCResult<String> {
        let chars = input.chars().collect::<Vec<char>>();

        for i in 4..chars.len() {
            let current = chars[i - 4..i].to_vec();
            let mut duplicates = Vec::new();
            let mut dup = false;
            for c in current.iter() {
                if duplicates.contains(c) {
                    dup = true
                }
                duplicates.push(c.to_owned());
            }

            if !dup {
                return Ok(i.to_string());
            }
        }

        Ok((-1).to_string())
    }

    fn run_pt2(input: String) -> AOCResult<String> {
        let chars = input.chars().collect::<Vec<char>>();

        for i in 14..chars.len() {
            let current = chars[i - 14..i].to_vec();
            let mut duplicates = Vec::new();
            let mut dup = false;
            for c in current.iter() {
                if duplicates.contains(c) {
                    dup = true
                }
                duplicates.push(c.to_owned());
            }

            if !dup {
                return Ok(i.to_string());
            }
        }

        Ok((-1).to_string())
    }
}
