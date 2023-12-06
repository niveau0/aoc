use std::{env, fs, path::Path};

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn count_best_bets(&self) -> u64 {
        (0..=self.time)
            .filter(|pressed| (self.time - pressed) * pressed > self.distance)
            .count() as u64
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let races: Vec<Race> = data
        .split_once("\n")
        .map(|(t, d)| (t.split_once(':').unwrap().1, d.split_once(':').unwrap().1))
        .map(|(t, d)| {
            (
                t.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
                d.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<u64>>(),
            )
        })
        .map(|(t, d)| {
            t.iter()
                .zip(d.iter())
                .map(|(t, d)| Race {
                    time: *t,
                    distance: *d,
                })
                .collect::<Vec<_>>()
        })
        .unwrap();

    println!("## Part 1");
    part1(&races);

    println!("## Part 2");
    let (real_time, real_distance) = races.iter().fold((String::new(), String::new()), |acc, r| {
        (
            format!("{}{}", acc.0, r.time),
            format!("{}{}", acc.1, r.distance),
        )
    });
    let real_race = Race {
        time: real_time.parse().unwrap(),
        distance: real_distance.parse().unwrap(),
    };
    part2(&real_race);
}

fn part1(races: &[Race]) {
    let product: u64 = races.iter().map(|race| race.count_best_bets()).product();

    println!("Part1: {:?}", product);
}

fn part2(race: &Race) {
    println!("Part1: {:?}", race.count_best_bets());
}
