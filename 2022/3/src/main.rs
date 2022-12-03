use std::{
    collections::{HashMap, HashSet},
    env, fs,
    path::Path,
};

trait Prio {
    fn prio(&self) -> u32;
}

impl Prio for char {
    fn prio(&self) -> u32 {
        let v = *self as u32;
        if v >= 96 {
            v - 96
        } else {
            v - 38
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let input: Vec<&str> = data.split("\n").filter(|l| !l.is_empty()).collect();

    part1(&input);
    part2(&input);
}

fn part1(content: &Vec<&str>) {
    let total = content.iter().fold(0, |acc, c| {
        let (first, second) = c.split_at(c.len() / 2);
        let mut first_items = HashSet::new();
        first.chars().for_each(|c| {
            first_items.insert(c);
        });
        second
            .chars()
            .find(|c| first_items.contains(&c))
            .map(|c| acc + c.prio())
            .expect("No duplicate!")
    });
    println!("## Part 1");
    println!("{}", total);
}

fn part2(content: &Vec<&str>) {
    let total = content.chunks(3).fold(0, |acc, group| {
        let (first, second, third) = (group[0], group[1], group[2]);
        let mut mem = HashMap::<char, u32>::new();
        first.chars().for_each(|c| {
            mem.insert(c, 1);
        });
        second.chars().for_each(|c| {
            if mem.contains_key(&c) {
                mem.insert(c, 2);
            }
        });
        third
            .chars()
            .find(|c| mem.contains_key(&c) && *mem.get(&c).unwrap() == 2u32)
            .map(|c| acc + c.prio())
            .expect("No badge!")
    });
    println!("## Part 2");
    println!("{}", total);
}
