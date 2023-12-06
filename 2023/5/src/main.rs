use std::{env, fs, ops::Range, path::Path};

#[derive(Debug)]
struct MapRange {
    dest: u64,
    src: u64,
    len: usize,
}
impl MapRange {
    fn split_by_range(&self, range: &Range<u64>) -> (Vec<Range<u64>>, bool) {
        let mut new_ranges = Vec::new();

        let map_range = (
            range.start.max(self.src),
            range.end.min(self.src + self.len as u64),
        );

        if map_range.0 < map_range.1 {
            if map_range.0 > range.start {
                new_ranges.push(range.start..map_range.0);
            }
            new_ranges.push(map_range.0 - self.src + self.dest..map_range.1 - self.src + self.dest);
            if range.end > map_range.1 {
                new_ranges.push(map_range.1..range.end);
            }
            println!("{:?} -> {:?} via {:?}", &range, &new_ranges, self);
            // if new_ranges.len() == 3 {
            //     panic!()
            // }
            (new_ranges, false)
        } else {
            // outside of range, keep numbers
            new_ranges.push(range.clone());
            println!("{:?} -> {:?} via {:?}", &range, &new_ranges, self);
            (new_ranges, true)
        }
    }
}

#[derive(Debug)]
pub struct Mapper {
    ranges: Vec<MapRange>,
}

impl Mapper {
    fn new() -> Self {
        Mapper { ranges: Vec::new() }
    }

    fn add_range(&mut self, dest: u64, src: u64, len: usize) {
        self.ranges.push(MapRange { dest, src, len });
    }

    fn map_value(&self, n: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|r| {
                if (r.src..r.src + (r.len as u64)).contains(&n) {
                    Some(n - r.src + r.dest)
                } else {
                    None
                }
            })
            .unwrap_or(n)
    }
    fn map_range(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        self.ranges
            .iter()
            .map(|r| r.split_by_range(range))
            .find(|(x, b)| *b)
            .map(|(x, _)| x)
            .unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut data = data.split('\n').filter(|l| !l.is_empty());
    let seeds = data
        .next()
        .map(|l| l.split_once(':').unwrap())
        .map(|(_, s)| {
            s.split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .unwrap();
    let mut mapper: Vec<Mapper> = data
        .filter(|l| !l.is_empty())
        .fold(Vec::new(), |mut acc, l| {
            if l.contains(':') {
                acc.push(Mapper::new());
            } else {
                let parts: Vec<u64> = l
                    .split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect();
                if parts.len() != 3 {
                    panic!("Invalid range")
                }
                acc.last_mut()
                    .unwrap()
                    .add_range(parts[0], parts[1], parts[2] as usize)
            }
            acc
        });

    println!("## Part 1");
    part1(&seeds, &mapper);

    println!("## Part 2");
    let seed_ranges = seeds
        .chunks(2)
        .map(|chunk| (chunk[0]..chunk[0] + chunk[1]))
        .collect::<Vec<Range<u64>>>();
    part2(&seed_ranges, &mut mapper);
}

fn part1(seeds: &[u64], mapper: &[Mapper]) {
    let min = seeds
        .iter()
        .map(|s| mapper.iter().fold(*s, |acc, m| m.map_value(acc)))
        .min()
        .unwrap();
    println!("Part1: {}", min);
}

fn part2(seed_ranges: &[Range<u64>], mapper: &[Mapper]) {
    // brute force (took hours):
    dbg!(seed_ranges.len());
    let min = seed_ranges
        .iter()
        .enumerate()
        .map(|(i, r)| {
            dbg!(i);
            r.clone()
                .map(|s| mapper.iter().fold(s, |acc, m| m.map_value(acc)))
                .min()
                .unwrap()
        })
        .min()
        .unwrap();

    println!("Part2: {}", min);

    // Too bad: there is a bug I fail find for the better solution:

    // let mut seed_ranges: HashSet<Range<u64>> = seed_ranges.iter().cloned().collect();
    // println!("{:?}", &seed_ranges);
    // mapper.iter().for_each(|m| {
    //     seed_ranges = seed_ranges.iter().flat_map(|r| m.map_range(r)).collect();
    //     println!("{:?}", &seed_ranges);
    // });

    // println!(
    //     "Part2: {}",
    //     seed_ranges.iter().map(|r| r.start).min().unwrap()
    // );
}
