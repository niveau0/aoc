use std::{collections::HashMap, env, fs, path::Path};

#[derive(Debug)]
pub struct Card {
    copies: u32,
    winning: HashMap<u8, bool>,
    numbers: Vec<u8>,
}

impl Card {
    fn new(winning: Vec<u8>, numbers: Vec<u8>) -> Self {
        Card {
            copies: 1,
            winning: winning.iter().map(|n| (*n, true)).collect(),
            numbers,
        }
    }

    fn add_copies(&mut self, copies: u32) {
        self.copies += copies;
    }

    fn matches(&self) -> usize {
        self.numbers.iter().fold(0, |a, n| {
            if self.winning.contains_key(n) {
                a + 1
            } else {
                a
            }
        })
    }
    fn value(&self) -> u32 {
        let matches = self.matches();
        if matches > 0 {
            1 << (matches - 1)
        } else {
            0
        }
    }

    fn copies(&self) -> u32 {
        self.copies
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut data: Vec<Card> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once('|').unwrap())
        .map(|(l, r)| {
            let (_, w) = l.split_once(':').unwrap();
            let w = w.split_whitespace();
            let n = r.split_whitespace();
            Card::new(
                w.map(|s| s.parse::<u8>().unwrap()).collect(),
                n.map(|s| s.parse::<u8>().unwrap()).collect(),
            )
        })
        .collect();

    println!("## Part 1");
    part1(&data);

    println!("## Part 2");
    part2(&mut data);
}

fn part1(data: &[Card]) {
    println!("Part1: {}", data.iter().map(|c| { c.value() }).sum::<u32>());
}

fn part2(data: &mut [Card]) {
    let len = data.len();
    for i in 0..len {
        let matches = data[i].matches();
        let copies = data[i].copies();
        (0..matches).filter(|j| j + 1 + i < len).for_each(|j| {
            data[j + 1 + i].add_copies(copies);
        });
    }
    println!("Part2: {}", data.iter().map(|c| c.copies()).sum::<u32>());
}
