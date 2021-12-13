use std::{env, fs, path::Path};

#[derive(Debug, Clone)]
struct Input(Vec<Vec<bool>>, Vec<String>);

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");

    let lines: Vec<&str> = data.split("\n").map(|l| l.trim()).collect();
    let mut field = vec![vec![false; 2000]; 2000];
    let mut folds = vec![];
    let lineiter = lines.into_iter();
    let mut p2 = false;
    for l in lineiter {
        if l.is_empty() {
            p2 = true;
        } else {
            if p2 && l.starts_with("fold along ") {
                folds.push(l[11..].to_owned());
            }
            l.split_once(",").map(|(r, l)| {
                field[l.parse::<usize>().unwrap()][r.parse::<usize>().unwrap()] = true;
            });
        }
    }

    let input = Input(field, folds);
    part1(&input);
    part2(&input);
}

fn part1(input: &Input) {
    let mut input = input.clone();

    input.print();
    input.fold(input.1[0].clone());
    input.print();

    println!("## Part 1");
    println!("Result {}", input.count());
}

fn part2(input: &Input) {
    let mut input = input.clone();

    input.print();

    for fold in input.1.clone() {
        input.fold(fold);
    }
    input.print();

    println!("## Part 2");
    println!("Result {}", input.count());
}

impl Input {
    fn count(&self) -> usize {
        self.0
            .iter()
            .map(|c| c.iter().filter(|v| **v).count())
            .sum()
    }

    fn print(&mut self) {
        let (w, h) = (50, 20);
        (0..h).for_each(|r| {
            (0..w).for_each(|c| {
                if self.0[r][c] {
                    print!("\x1b[1m#")
                } else {
                    print!("\x1b[0m.")
                }
            });
            println!();
        });
        println!();
    }

    fn fold(&mut self, fold: String) {
        let (dir, pos) = fold.split_once("=").unwrap();
        let pos = pos.parse::<usize>().unwrap();
        if dir == "x" {
            for r in 0..self.0.len() {
                for c in 0..pos {
                    if self.0[r][pos * 2 - c] {
                        self.0[r][pos * 2 - c] = false;
                        self.0[r][c] = true;
                    }
                }
            }
        } else {
            let cols = self.0[0].len();
            for r in 0..pos {
                for c in 0..cols {
                    if self.0[pos * 2 - r][c] {
                        self.0[pos * 2 - r][c] = false;
                        self.0[r][c] = true;
                    }
                }
            }
        }
    }
}
