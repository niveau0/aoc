use std::{env, fs, path::Path};

const NEIGHBOURS: [(i8, i8); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

#[derive(Debug, Clone)]
struct Input(Vec<Vec<(u8, bool)>>, u64);

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    let runs: usize = args
        .get(2)
        .expect("Missing runs parameter")
        .parse()
        .unwrap();
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");

    let input = Input(
        data.split("\n")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.split("")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| (s.parse::<u8>().unwrap(), false))
                    .collect::<Vec<(u8, bool)>>()
            })
            .collect(),
        0,
    );

    part1(&input, runs);
    part2(&input);
}

fn part1(input: &Input, runs: usize) {
    let mut input = input.clone();

    input.print();
    (0..runs).for_each(|_| {
        input.step();
        input.print();
        input.reset();
    });

    println!("## Part 1");
    println!("Result {}", input.1);
}

fn part2(input: &Input) {
    let mut count = 0;
    let mut input = input.clone();

    loop {
        count += 1;
        input.step();
        if input.1 == 100 {
            input.print();
            break;
        }
        input.reset();
        input.1 = 0;
    }

    println!("## Part 2");
    println!("Result {}", count);
}

impl Input {
    fn print(&mut self) {
        let (w, h) = (10, 10);
        (0..h).for_each(|r| {
            (0..w).for_each(|c| {
                let (v, flashes) = self.0[r][c];
                if flashes {
                    print!("\x1b[1m{}", v)
                } else {
                    print!("\x1b[0m{}", v)
                }
            });
            println!();
        });
        println!();
    }

    fn reset(&mut self) {
        let (w, h) = (10, 10);

        (0..h).for_each(|r| {
            (0..w).for_each(|c| {
                self.0[r][c] = (self.0[r][c].0, false);
            });
        });
    }

    fn step(&mut self) {
        let (w, h) = (10, 10);

        (0..h).for_each(|r| {
            (0..w).for_each(|c| {
                self.inc_cell(c, r);
            });
        });
    }

    fn inc_cell(&mut self, c: usize, r: usize) {
        match self.0[r][c] {
            (0, true) => (),
            (9, _) => {
                self.0[r][c] = (0, true);
                self.1 += 1;
                self.inc_neighbours(c, r);
            }
            (v, _) => self.0[r][c] = (v + 1, false),
        };
    }

    fn inc_neighbours(&mut self, c: usize, r: usize) {
        let (w, h) = (10, 10);
        NEIGHBOURS.iter().for_each(|(x, y)| {
            let nx = c as i8 + x;
            let ny = r as i8 + y;
            if nx >= 0 && nx < w as i8 && ny >= 0 && ny < h as i8 {
                self.inc_cell(nx as usize, ny as usize);
            }
        });
    }
}
