use std::{env, fs, path::Path};

// code structure ideas taken from https://rustwasm.github.io/book/game-of-life/implementing.html
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Floor,
    Empty,
    Occupied,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new(file: &String) -> Self {
        let data = fs::read_to_string(file).expect("Something went wrong reading the file");

        let mut cells = vec![];
        let mut width = 0;
        let mut height = 0;
        data.split("\n").for_each(|l| {
            let l = l.trim();
            height = height + 1;
            width = l.len() as u32;
            l.chars().for_each(|c| match c {
                '.' => cells.push(Cell::Floor),
                'L' => cells.push(Cell::Empty),
                '#' => cells.push(Cell::Occupied),
                _ => panic!("Unknown cell type"),
            })
        });

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn seen_occupied(&self, row: u32, column: u32, max_dist: u32) -> u8 {
        let mut visited: [Cell; 9] = [Cell::Floor; 9];

        if self.cells[self.get_index(row, column)] == Cell::Floor {
            return 0;
        }

        for dist in 1..=max_dist {
            for delta_row in [-1, 0, 1].iter().cloned() {
                for delta_col in [-1, 0, 1].iter().cloned() {
                    let seen_row = row as i32 + delta_row * dist as i32;
                    let seen_col = column as i32 + delta_col * dist as i32;

                    match (seen_row, seen_col) {
                        (r, c) if r == row as i32 && c == column as i32 => continue,
                        (r, _) if r < 0 => break,
                        (r, _) if r >= self.height as i32 => break,
                        (_, c) if c < 0 => continue,
                        (_, c) if c >= self.width as i32 => break,
                        _ => {
                            let idx = self.get_index(seen_row as u32, seen_col as u32);
                            let visit_idx = ((delta_row + 1) * 3 + delta_col + 1) as usize;
                            match self.cells[idx] {
                                Cell::Empty if visited[visit_idx] == Cell::Floor => {
                                    visited[visit_idx] = Cell::Empty
                                }
                                Cell::Occupied if visited[visit_idx] == Cell::Floor => {
                                    visited[visit_idx] = Cell::Occupied
                                }
                                _ => (),
                            };
                        }
                    }
                }
            }
        }

        (0..9usize)
            .filter(|idx| visited[*idx] == Cell::Occupied)
            .map(|_| 1)
            .sum()
    }

    pub fn print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                match self.cells[idx] {
                    Cell::Floor => print!("."),
                    Cell::Empty => print!("L"),
                    Cell::Occupied => print!("#"),
                };
            }
            println!();
        }
    }

    pub fn tick(&mut self, max_dist: u32, seats: u8) -> u32 {
        let mut next = self.cells.clone();
        let mut changes = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let neighbors = self.seen_occupied(row, col, max_dist);
                let next_cell = match (cell, neighbors) {
                    (Cell::Empty, 0) => Cell::Occupied,
                    (Cell::Occupied, c) if c >= seats => Cell::Empty,
                    (cell, _) => cell,
                };
                if cell != next_cell {
                    changes = changes + 1;
                }

                next[idx] = next_cell;
            }
        }
        self.cells = next;

        changes
    }

    pub fn count(&self, state: Cell) -> u32 {
        let mut count = 0;
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                if self.cells[idx] == state {
                    count = count + 1;
                }
            }
        }
        count
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let mut universe = Universe::new(file);
    println!(
        "Universe width {}, height {}",
        universe.width(),
        universe.height()
    );

    part1(&mut universe);

    let mut universe = Universe::new(file);
    part2(&mut universe);
}

fn part1(universe: &mut Universe) {
    loop {
        if universe.tick(1, 4) == 0 {
            break;
        }
    }

    println!("## Part 1");
    println!("{}", universe.count(Cell::Occupied))
}

fn part2(universe: &mut Universe) {
    let max_dist = std::cmp::min(universe.height(), universe.width());
    loop {
        if universe.tick(max_dist, 5) == 0 {
            break;
        }
    }
    println!("## Part 2");
    println!("{}", universe.count(Cell::Occupied))
}
