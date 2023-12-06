use std::{env, fs, ops::Range, path::Path};

#[derive(Debug)]
struct MapRange {
    dest: u64,
    src: u64,
    len: usize,
}
impl MapRange {
    // returns not-mapped ranges and mapped range
    fn split_by_range_and_map(&self, range: &Range<u64>) -> (Vec<Range<u64>>, Option<Range<u64>>) {
        let mut kept_ranges = Vec::new();
        let mapped_range;

        let map_range = (
            range.start.max(self.src),
            range.end.min(self.src + self.len as u64),
        );

        if map_range.0 < map_range.1 {
            mapped_range =
                Some(map_range.0 - self.src + self.dest..map_range.1 - self.src + self.dest);
            if map_range.0 > range.start {
                kept_ranges.push(range.start..map_range.0);
            }
            if range.end > map_range.1 {
                kept_ranges.push(map_range.1..range.end);
            }
        } else {
            // outside of range, keep numbers
            mapped_range = None;
        }
        (kept_ranges, mapped_range)
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
    fn map_range(&self, seed_range: &Range<u64>) -> Vec<Range<u64>> {
        let mut seed_ranges = vec![seed_range.clone()];
        let mut new_ranges = vec![];
        while !seed_ranges.is_empty() {
            if let Some(range) = &seed_ranges.pop() {
                let mut was_mapped = false;
                for map_range in &self.ranges {
                    let (kept, mapped) = map_range.split_by_range_and_map(&range);
                    if let Some(mapped) = mapped {
                        new_ranges.push(mapped);
                        kept.into_iter().for_each(|r| seed_ranges.push(r));
                        was_mapped = true;
                        break;
                    }
                }
                if !was_mapped {
                    new_ranges.push(range.clone());
                }
            }
        }
        new_ranges
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
    // dbg!(seed_ranges.len());
    // let min = seed_ranges
    //     .iter()
    //     .enumerate()
    //     .map(|(i, r)| {
    //         dbg!(i);
    //         r.clone()
    //             .map(|s| mapper.iter().fold(s, |acc, m| m.map_value(acc)))
    //             .min()
    //             .unwrap()
    //     })
    //     .min()
    //     .unwrap();

    // Too bad: there is a bug I fail find for the better solution:

    let mut sr = seed_ranges.to_vec();
    for m in mapper {
        sr = sr.iter().flat_map(|r| m.map_range(r)).collect();
    }

    println!("Part2: {}", sr.iter().map(|r| r.start).min().unwrap());
}
