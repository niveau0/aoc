use std::{env, fs, path::Path};

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

type Pair = ((u16, u16), (u16, u16));

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let input: Vec<Pair> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| line.split(','))
        .map(|mut pairs| {
            (
                pairs.next().unwrap().split('-'),
                pairs.next().unwrap().split('-'),
            )
        })
        .map(|mut ranges| {
            (
                (
                    ranges.0.next().unwrap().parse().unwrap(),
                    ranges.0.next().unwrap().parse().unwrap(),
                ),
                (
                    ranges.1.next().unwrap().parse().unwrap(),
                    ranges.1.next().unwrap().parse().unwrap(),
                ),
            )
        })
        .collect();

    part1(&input);
    part2(&input);
}

fn part1(ranges: &[Pair]) {
    let total: u16 = ranges
        .iter()
        .map(|pair| {
            let l = pair.0;
            let r = pair.1;
            u16::from((l.0 <= r.0 && l.1 >= r.1) || (r.0 <= l.0 && r.1 >= l.1))
        })
        .sum();
    println!("## Part 1");
    println!("{}", total);
}

fn part2(ranges: &[Pair]) {
    let total: u16 = ranges
        .iter()
        .map(|pair| {
            let l = pair.0;
            let r = pair.1;
            u16::from(
                (l.0 <= r.0 && l.1 >= r.0)
                    || (r.0 <= l.0 && r.1 >= l.0)
                    || (l.0 <= r.1 && l.1 >= r.1)
                    || (r.0 <= l.1 && r.1 >= l.1),
            )
        })
        .sum();
    println!("## Part 2");
    println!("{}", total);
}
