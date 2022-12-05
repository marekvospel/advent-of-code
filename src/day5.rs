use crate::{AOCResult, AOCRunnable};

pub struct AOCDay;

#[derive(Debug, Copy, Clone)]
struct Crate {
    identifier: char,
}

impl Crate {
    fn parse<S: Into<String>>(input: S) -> Option<Self> {
        let input = input.into();

        let identifier = input.chars().nth(1)?;

        match identifier {
            'A'..='Z' => Some(Crate { identifier }),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Pile {
    index: usize,
    pub(crate) pile: Vec<Crate>,
}

impl Pile {
    fn new(index: usize) -> Self {
        Pile {
            index,
            pile: Vec::new(),
        }
    }

    fn push(&mut self, cr: Crate) {
        self.pile.push(cr)
    }

    fn take(&mut self, amount: usize) -> Vec<Crate> {
        let mut vec = Vec::new();
        for _ in 0..amount {
            let res = self.pile.remove(self.pile.len() - 1);
            vec.push(res);
        }

        vec
    }

    fn top(&self) -> Option<&Crate> {
        self.pile.last()
    }
}

impl AOCRunnable for AOCDay {
    fn run_pt1(input: String) -> AOCResult<String> {
        let input: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();

        let schema = input.first().expect("schema should exist");
        let schema_split = schema.split('\n');
        let piles_len = schema_split
            .clone()
            .last()
            .expect("last row in shema should exist")
            .split("   ")
            .count();

        println!("Piles len: {}", piles_len);

        let mut columns = Vec::new();

        for i in 0..piles_len {
            columns.push(Pile::new(i));
        }

        // println!("{}", columns.len());
        // println!("{:?}", columns);

        let mut piles_packages = schema_split
            .clone()
            .take(schema_split.count() - 1)
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        piles_packages.reverse();

        for row in piles_packages.iter() {
            for column in columns.iter_mut() {
                let pkg: String = row.clone().chars().skip(column.index * 4).take(3).collect();

                if let Some(c) = Crate::parse(pkg.clone()) {
                    column.push(c);
                }
            }
        }

        let instructions = input.get(1).expect("instructions should exist");

        for instruction in instructions.split('\n') {
            let num = instruction.split(' ').nth(1);
            if num.is_none() {
                continue;
            }
            let num = num.unwrap().parse::<i32>()?;

            let from = instruction.split(' ').nth(3);
            if from.is_none() {
                continue;
            }
            let from = from.unwrap().parse::<i32>()?;

            let to = instruction.split(' ').nth(5);
            if to.is_none() {
                continue;
            }
            let to = to.unwrap().parse::<i32>()?;

            for _ in 0..num {
                let taken;
                {
                    let from_col = columns.get_mut(from as usize - 1).unwrap();
                    taken = from_col.take(1);
                }
                {
                    let to_col = columns.get_mut(to as usize - 1).unwrap();
                    for t in taken {
                        to_col.push(t);
                    }
                }
            }
        }

        let result = columns
            .iter()
            .map(|c| c.top().unwrap_or(&Crate { identifier: ' ' }))
            .collect::<Vec<&Crate>>();

        Ok(result.iter().map(|c| c.identifier).collect::<String>())
    }

    fn run_pt2(input: String) -> AOCResult<String> {
        let input: Vec<String> = input.split("\n\n").map(|s| s.to_string()).collect();

        let schema = input.first().expect("schema should exist");
        let schema_split = schema.split('\n');
        let piles_len = schema_split
            .clone()
            .last()
            .expect("last row in shema should exist")
            .split("   ")
            .count();

        println!("Piles len: {}", piles_len);

        let mut columns = Vec::new();

        for i in 0..piles_len {
            columns.push(Pile::new(i));
        }

        // println!("{}", columns.len());
        // println!("{:?}", columns);

        let mut piles_packages = schema_split
            .clone()
            .take(schema_split.count() - 1)
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        piles_packages.reverse();

        for row in piles_packages.iter() {
            for column in columns.iter_mut() {
                let pkg: String = row.clone().chars().skip(column.index * 4).take(3).collect();

                if let Some(c) = Crate::parse(pkg.clone()) {
                    column.push(c);
                }
            }
        }

        let instructions = input.get(1).expect("instructions should exist");

        for instruction in instructions.split('\n') {
            let num = instruction.split(' ').nth(1);
            if num.is_none() {
                continue;
            }
            let num = num.unwrap().parse::<i32>()?;

            let from = instruction.split(' ').nth(3);
            if from.is_none() {
                continue;
            }
            let from = from.unwrap().parse::<i32>()?;

            let to = instruction.split(' ').nth(5);
            if to.is_none() {
                continue;
            }
            let to = to.unwrap().parse::<i32>()?;

            let mut taken;
            {
                let from_col = columns.get_mut(from as usize - 1).unwrap();
                taken = from_col.take(num as usize);
                taken.reverse();
            }
            {
                let to_col = columns.get_mut(to as usize - 1).unwrap();
                for t in taken {
                    to_col.push(t);
                }
            }
        }

        let result = columns
            .iter()
            .map(|c| c.top().unwrap_or(&Crate { identifier: ' ' }))
            .collect::<Vec<&Crate>>();

        Ok(result.iter().map(|c| c.identifier).collect::<String>())
    }
}
