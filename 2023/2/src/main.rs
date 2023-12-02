use std::{collections::HashMap, env, fs, path::Path};

#[derive(Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

impl Set {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Set { red, green, blue }
    }

    fn allowed_with_config(&self, config: &Set) -> bool {
        self.red <= config.red && self.green <= config.green && self.blue <= config.blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl From<&str> for Set {
    fn from(value: &str) -> Self {
        let mut set = Set::new(0, 0, 0);
        value
            .split(',')
            .map(|s| s.trim().split_once(' ').unwrap())
            .for_each(|(v, c)| match (v.parse::<u32>().unwrap(), c) {
                (v, "red") => set.red = v,
                (v, "green") => set.green = v,
                (v, "blue") => set.blue = v,
                _ => panic!("unknown color"),
            });

        set
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: HashMap<u32, Vec<Set>> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(':').unwrap())
        .map(|(game, sets)| (game[5..].parse::<u32>().unwrap(), sets.split(';')))
        .map(|(gameid, sets)| (gameid, sets.into_iter().map(Set::from).collect()))
        .collect();

    println!("## Part 1");
    part1(&data, Set::new(12, 13, 14));

    println!("## Part 2");
    part2(&data);
}

fn part1(data: &HashMap<u32, Vec<Set>>, config: Set) {
    let sum = data.iter().fold(0, |a, (id, sets)| {
        if sets.iter().all(|s| s.allowed_with_config(&config)) {
            a + id
        } else {
            a
        }
    });

    println!("Part1: {}", sum);
}

fn part2(data: &HashMap<u32, Vec<Set>>) {
    let sum = data.iter().fold(0, |a, (_id, sets)| {
        let min = sets.iter().fold(Set::new(0, 0, 0), |mut a, s| {
            a.red = s.red.max(a.red);
            a.green = s.green.max(a.green);
            a.blue = s.blue.max(a.blue);
            a
        });
        a + min.power()
    });

    println!("Part2: {}", sum);
}
