use std::{env, fs, path::Path};

#[derive(Debug, Clone)]
struct Heatmap(Vec<Vec<(u8, bool)>>);

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");

    let input: Vec<Vec<(u8, bool)>> = data
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split("")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| (s.parse::<u8>().unwrap(), false))
                .collect::<Vec<(u8, bool)>>()
        })
        .collect();

    part1(Heatmap(input.clone()));
    part2(Heatmap(input.clone()));
}

fn part1(mut input: Heatmap) {
    let (rows, cols) = input.size();
    (0..rows).for_each(|r| (0..cols).for_each(|c| input.mark_valley(r, c)));
    let sum: u32 = (0..rows)
        .map(|r| {
            (0..cols)
                .filter(|c| input.is_marked(r, *c))
                .map(|c| input.get(r, c) as u32 + 1)
                .sum::<u32>()
        })
        .sum();
    input.print();
    println!("## Part 1");
    println!("Result {}", sum);
}

fn part2(mut input: Heatmap) {
    let (rows, cols) = input.size();
    (0..rows).for_each(|r| (0..cols).for_each(|c| input.mark_valley(r, c)));

    let valley_pos: Vec<(usize, usize)> = (0..rows)
        .flat_map(|r| {
            (0..cols)
                .filter(|c| input.is_marked(r, *c))
                .map(|c| (r, c))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    input.mark_basins();
    input.print();

    let (a, b, c) = valley_pos
        .iter()
        .map(|(r, c)| input.basin_size(*r, *c))
        .fold((0, 0, 0), |(a, b, c), s| {
            if s > a {
                (s, a, b)
            } else if s > b {
                (a, s, b)
            } else if s > c {
                (a, b, s)
            } else {
                (a, b, c)
            }
        });
    println!("## Part 2");
    println!("Result {}", a * b * c);
}

impl Heatmap {
    fn print(&mut self) {
        let (h, w) = self.size();
        (0..h).for_each(|r| {
            (0..w).for_each(|c| {
                if self.is_marked(r, c) {
                    print!("\x1b[1m{}", self.get(r, c))
                } else {
                    print!("\x1b[0m{}", self.get(r, c))
                }
            });
            println!();
        });
    }

    fn size(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }

    fn mark_valley(&mut self, row: usize, col: usize) {
        let mid = self.get(row, col);
        let (h, w) = self.size();
        if row > 0 && self.get(row - 1, col) <= mid {
            return;
        }
        if row < h - 1 && self.get(row + 1, col) <= mid {
            return;
        }
        if col > 0 && self.get(row, col - 1) <= mid {
            return;
        }
        if col < w - 1 && self.get(row, col + 1) <= mid {
            return;
        }
        self.0[row][col].1 = true;
    }

    fn get(&self, row: usize, col: usize) -> u8 {
        self.0[row][col].0
    }

    fn is_marked(&self, row: usize, col: usize) -> bool {
        self.0[row][col].1
    }

    fn mark_basins(&mut self) {
        let (h, w) = self.size();
        let change = (0..h)
            .map(|r| {
                (0..w)
                    .map(|c| {
                        if self.is_marked(r, c) {
                            self.mark_higher_environment_but_9(r, c)
                        } else {
                            false
                        }
                    })
                    .reduce(|a, b| a | b)
                    .unwrap()
            })
            .reduce(|a, b| a | b)
            .unwrap();
        if change {
            self.mark_basins()
        }
    }

    fn mark_higher_environment_but_9(&mut self, row: usize, col: usize) -> bool {
        let mid = self.get(row, col);
        let (h, w) = self.size();
        let mut change = false;

        if row > 0 {
            change |= self.mark_higher_but_9(row - 1, col, mid);
        }
        if row < h - 1 {
            change |= self.mark_higher_but_9(row + 1, col, mid);
        }
        if col > 0 {
            change |= self.mark_higher_but_9(row, col - 1, mid);
        }
        if col < w - 1 {
            change |= self.mark_higher_but_9(row, col + 1, mid);
        }
        change
    }

    fn mark_higher_but_9(&mut self, row: usize, col: usize, mid: u8) -> bool {
        if self.get(row, col) < 9 && self.get(row, col) > mid && !self.0[row][col].1 {
            self.0[row][col].1 = true;
            true
        } else {
            false
        }
    }

    fn basin_size(&mut self, r: usize, c: usize) -> u32 {
        let (h, w) = self.size();
        let mut sum = 0;
        if r > 0 && self.is_marked(r - 1, c) {
            self.0[r - 1][c].1 = false;
            sum += 1 + self.basin_size(r - 1, c);
        }
        if r < h - 1 && self.is_marked(r + 1, c) {
            self.0[r + 1][c].1 = false;
            sum += 1 + self.basin_size(r + 1, c);
        }
        if c > 0 && self.is_marked(r, c - 1) {
            self.0[r][c - 1].1 = false;
            sum += 1 + self.basin_size(r, c - 1);
        }
        if c < w - 1 && self.is_marked(r, c + 1) {
            self.0[r][c + 1].1 = false;
            sum += 1 + self.basin_size(r, c + 1);
        }

        sum
    }
}
