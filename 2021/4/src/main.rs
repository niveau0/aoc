use std::{env, fs, path::Path};

#[derive(Debug)]
struct Cell {
    mark: bool,
    value: i64,
}

#[derive(Debug)]
struct Field {
    cells: Vec<Cell>,
    winner: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let numsfile = args.get(2).expect("Missing file parameter");
    if !Path::new(numsfile).exists() {
        panic!("No such file {}", numsfile);
    }
    let fielddata = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut fields = vec![];
    fielddata
        .split("\n")
        .enumerate()
        .map(|(idx, l)| (idx, l.trim()))
        .filter(|(_, l)| !l.is_empty())
        .for_each(|(idx, l)| {
            if fields.len() <= idx / 6 {
                fields.push(Field {
                    cells: vec![],
                    winner: false,
                });
            };
            l.split(" ")
                .map(|n| n.trim())
                .filter(|n| !n.is_empty())
                .for_each(|n| {
                    fields[idx / 6].cells.push(Cell {
                        mark: false,
                        value: n.parse().unwrap(),
                    })
                });
        });

    let numsdata = fs::read_to_string(numsfile).expect("Something went wrong reading the file");
    let drawn: Vec<i64> = numsdata
        .split(",")
        .map(|l| l.trim())
        .map(|n| n.parse().unwrap())
        .collect();

    part1(&mut fields, &drawn);
    part2(&mut fields, &drawn);
}

fn part1(fields: &mut Vec<Field>, drawn: &Vec<i64>) {
    let winning_number = drawn
        .iter()
        .find(|&&n| {
            fields.iter_mut().fold(false, |result, f| {
                f.mark_numbers(n);
                f.mark_if_winner();
                f.is_winner() || result
            })
        })
        .unwrap();

    let winners: Vec<(usize, &Field)> = fields
        .iter()
        .enumerate()
        .filter(|(_, f)| f.is_winner())
        .collect();
    let sum: i64 = winners
        .iter()
        .map(|(_, f)| f.sum_unmarked())
        .collect::<Vec<i64>>()
        .iter()
        .sum();

    println!("## Part 1");
    println!(
        "Winners: {:?}, Sum: {}, Winning number: {}, Result: {}",
        winners.iter().map(|(idx, _)| *idx).collect::<Vec<usize>>(),
        sum,
        winning_number,
        sum * winning_number
    );
}

fn part2(fields: &mut Vec<Field>, drawn: &Vec<i64>) {
    let winning_numbers: Vec<&i64> = drawn
        .iter()
        .filter(|&&n| {
            fields
                .iter_mut()
                .filter(|f| !f.is_winner())
                .fold(false, |result, f| {
                    f.mark_numbers(n);
                    f.mark_if_winner();
                    f.is_winner() || result
                })
        })
        .collect();
    let last = **winning_numbers.last().unwrap();

    let last_winner = fields
        .iter()
        .find(|f| f.is_winner() && f.is_number_marked(last))
        .unwrap();

    let sum = last_winner.sum_unmarked();
    println!("## Part 2");
    println!(
        "Sum: {}, Last winning number: {}, Result: {}",
        sum,
        last,
        sum * last
    );
}

impl Field {
    fn is_winner(&self) -> bool {
        self.winner
    }
    fn mark_if_winner(&mut self) {
        let len = 5;
        self.winner = if self
            .cells
            .chunks(5)
            .find(|chunk| chunk.iter().all(|c| c.mark))
            .is_some()
        {
            true
        } else if (0..len)
            .find(|col| (0..len).all(|row| self.cells[col + len * row].mark))
            .is_some()
        {
            true
        } else {
            false
        }
    }

    fn mark_numbers(&mut self, drawn: i64) {
        self.cells
            .iter_mut()
            .filter(|c| c.value == drawn)
            .for_each(|c| c.mark = true);
    }

    fn sum_unmarked(&self) -> i64 {
        self.cells.iter().filter(|c| !c.mark).map(|c| c.value).sum()
    }

    fn is_number_marked(&self, number: i64) -> bool {
        self.cells
            .iter()
            .find(|c| c.mark && c.value == number)
            .is_some()
    }
}
