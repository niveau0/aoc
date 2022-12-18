use itertools::Itertools;
use std::{collections::HashMap, env, fs, io::Write, path::Path, time};

struct Cave {
    data: HashMap<(u16, u16), char>,
    min_x: u16,
    max_x: u16,
    max_y: u16,
    has_floor: bool,
}

impl Cave {
    fn new(data: HashMap<(u16, u16), char>) -> Self {
        let min_x = *data.iter().map(|((x, _), _)| x).min().unwrap();
        let max_x = *data.iter().map(|((x, _), _)| x).max().unwrap();
        let max_y = *data.iter().map(|((_, y), _)| y).max().unwrap();

        Cave {
            data,
            min_x,
            max_x,
            max_y,
            has_floor: false,
        }
    }

    fn print_letter(&self, x: u16, y: u16, letter: char) {
        let offset_x = self.min_x - 50;
        if x + 1 < offset_x {
            return;
        }
        print!(
            "{}{}",
            termion::cursor::Goto(x + 1 - offset_x, y + 1),
            letter
        );
    }
    fn print(&self) {
        let width = self.max_x - self.min_x + 100;
        let height = self.max_y + 2;
        let offset_x = self.min_x - 50;
        print!("{}", termion::clear::All);
        std::thread::sleep(time::Duration::from_millis(2000));
        (0..height).for_each(|r| {
            (0..width).for_each(|c| match self.data.get(&(c + offset_x, r)) {
                None => self.print_letter(c + offset_x, r, '.'),
                Some(l) => self.print_letter(c + offset_x, r, *l),
            })
        });
        print!("{}", termion::cursor::Goto(1, self.max_y + 4));
        println!();
    }

    fn pour_sand(&mut self) {
        let mut can_move = false;
        while !can_move {
            if !self.insert_grain(500, 0) {
                break;
            }
            can_move = self.move_grain((500, 0));
        }
        print!("{}", termion::cursor::Goto(1, self.max_y + 4));
        println!();
    }

    fn insert_grain(&mut self, x: u16, y: u16) -> bool {
        if self.data.get(&(x, y)).is_none() {
            self.data.insert((x, y), 'O');
            self.print_letter(x, y, 'O');
            std::io::stdout().flush().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(50));
            true
        } else {
            false
        }
    }

    fn move_grain(&mut self, start: (u16, u16)) -> bool {
        let (mut x, mut y) = start;
        loop {
            match self.has_floor {
                true if y + 1 == self.max_y + 2 => return false,
                false if y + 1 > self.max_y => {
                    self.data.remove(&(x, y));
                    return true;
                }
                _ => {
                    let (nx, ny) = if self.insert_grain(x, y + 1) {
                        (x, y + 1)
                    } else if self.insert_grain(x - 1, y + 1) {
                        (x - 1, y + 1)
                    } else if self.insert_grain(x + 1, y + 1) {
                        (x + 1, y + 1)
                    } else {
                        return false;
                    };
                    self.data.remove(&(x, y));
                    self.print_letter(x, y, '.');
                    (x, y) = (nx, ny)
                }
            }
        }
    }

    fn grain_count(&self) -> usize {
        self.data.iter().filter(|(_, l)| **l == 'O').count()
    }

    pub(crate) fn add_floor(&mut self) {
        self.has_floor = true;
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: HashMap<(u16, u16), char> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|s| s.split(" -> "))
        .flat_map(|pos_iter| {
            pos_iter
                .map(|pos| {
                    pos.split_once(',')
                        .map(|(x, y)| (x.parse::<u16>().unwrap(), y.parse::<u16>().unwrap()))
                        .unwrap()
                })
                .tuple_windows()
                .flat_map(|(from, to)| rock_path(from, to).into_iter())
        })
        .map(|pos| (pos, '#'))
        .collect();

    let mut cave = Cave::new(data);

    cave.print();
    cave.pour_sand();

    println!("## Part 1");
    println!("total: {}", cave.grain_count());

    cave.add_floor();
    cave.pour_sand();
    println!("## Part 2");
    println!("total: {}", cave.grain_count());
}

fn rock_path(from: (u16, u16), to: (u16, u16)) -> Vec<(u16, u16)> {
    let mut path: Vec<(u16, u16)> = vec![];
    for x in from.0.min(to.0)..=from.0.max(to.0) {
        for y in from.1.min(to.1)..=from.1.max(to.1) {
            path.push((x, y));
        }
    }
    path
}
