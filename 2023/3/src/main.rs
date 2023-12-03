use std::{env, fs, path::Path};

#[derive(Debug)]
struct EngineSchema {
    symbols: Vec<Vec<(usize, char)>>,
    numbers: Vec<Vec<(usize, usize, u64)>>,
}

impl EngineSchema {
    fn new(schema: Vec<Vec<(usize, char)>>) -> Self {
        let symbols = schema
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|(_i, c)| !c.is_numeric())
                    .map(|(i, c)| (*i, *c))
                    .collect()
            })
            .collect();
        let numbers: Vec<Vec<(usize, usize, u64)>> = schema
            .iter()
            .map(|line| {
                line.iter()
                    .filter(|(_i, c)| c.is_numeric())
                    .map(|(i, c)| (*i, c.to_digit(10).unwrap() as u64))
                    .collect::<Vec<(usize, u64)>>()
            })
            .map(|digits_in_line| {
                let mut numbers_in_line = Vec::new();
                let mut num = 0;
                let mut start = 0;
                let mut end = None;
                for (di, d) in digits_in_line.iter() {
                    match end {
                        None => {
                            start = *di;
                            end = Some(*di);
                            num = *d;
                        }
                        Some(e) if e == di - 1 => {
                            end = Some(*di);
                            num = num * 10 + d;
                        }
                        Some(e) => {
                            numbers_in_line.push((start, e, num));
                            start = *di;
                            end = Some(*di);
                            num = *d;
                        }
                    }
                }
                if let Some(e) = end {
                    numbers_in_line.push((start, e, num));
                }
                numbers_in_line
            })
            .collect();

        EngineSchema { symbols, numbers }
    }

    fn parts(&self) -> Vec<u64> {
        self.numbers
            .iter()
            .enumerate()
            .flat_map(|(number_line_idx, number_line)| {
                number_line
                    .iter()
                    .filter(|number| self.touches_symbol(number_line_idx, number))
                    .map(|(_, _, v)| *v)
                    .collect::<Vec<u64>>()
            })
            .collect()
    }

    fn gear_ratios(&self) -> Vec<u64> {
        self.symbols
            .iter()
            .enumerate()
            .flat_map(|(symbol_line_idx, symbol_line)| {
                symbol_line
                    .iter()
                    .filter(|(_i, c)| *c == '*')
                    .map(|(symbol_idx, _c)| self.gear_ratio(symbol_line_idx, *symbol_idx))
                    .collect::<Vec<u64>>()
            })
            .collect()
    }

    fn gear_ratio(&self, gear_line_idx: usize, gear_idx: usize) -> u64 {
        let adjacent: Vec<u64> = self
            .numbers
            .iter()
            .enumerate()
            .flat_map(|(number_line_idx, number_line)| {
                number_line
                    .iter()
                    .filter(|_| number_line_idx.abs_diff(gear_line_idx) < 2)
                    .filter(|(start, end, _)| number_range(start, end).contains(&gear_idx))
                    .map(|(_, _, n)| *n)
                    .collect::<Vec<u64>>()
            })
            .collect();
        if adjacent.len() == 2 {
            adjacent[0] * adjacent[1]
        } else {
            0
        }
    }

    fn touches_symbol(
        &self,
        number_line_idx: usize,
        (start, end, _number): &(usize, usize, u64),
    ) -> bool {
        let range = number_range(start, end);

        self.symbols.iter().enumerate().any(|(line_idx, line)| {
            line.iter().any(|(symbol_idx, _symbol)| {
                number_line_idx.abs_diff(line_idx) < 2 && range.contains(symbol_idx)
            })
        })
    }
}

fn number_range(start: &usize, end: &usize) -> std::ops::RangeInclusive<usize> {
    if *start == 0 {
        *start..=*end + 1
    } else {
        *start - 1..=*end + 1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<Vec<(usize, char)>> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().enumerate().filter(|(_, c)| *c != '.').collect())
        .collect();

    let schema = EngineSchema::new(data);

    println!("## Part 1");
    part1(&schema);

    println!("## Part 2");
    part2(&schema);
}

fn part1(schema: &EngineSchema) {
    let parts = schema.parts();
    let sum = parts.iter().sum::<u64>();
    println!("Part1: {}", sum);
}

fn part2(schema: &EngineSchema) {
    println!("Part2: {}", schema.gear_ratios().iter().sum::<u64>());
}
