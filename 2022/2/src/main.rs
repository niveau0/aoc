use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let pairs: Vec<(&str, &str)> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.split(' '))
        .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
        .collect();

    part1(&pairs);
    part2(&pairs);
}

fn part1(pairs: &[(&str, &str)]) {
    let total = pairs.iter().fold(0, |total, p| {
        let score = match p {
            ("A", "X") => 1 + 3,
            ("A", "Y") => 2 + 6,
            ("A", "Z") => 3,
            ("B", "X") => 1,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 1 + 6,
            ("C", "Y") => 2,
            ("C", "Z") => 3 + 3,
            _ => panic!("Illegal pair"),
        };
        total + score
    });

    println!("## Part 1");
    println!("{}", total);
}

fn part2(pairs: &[(&str, &str)]) {
    let total = pairs.iter().fold(0, |total, p| {
        let score = match p {
            ("A", "X") => 3,
            ("A", "Y") => 1 + 3,
            ("A", "Z") => 2 + 6,
            ("B", "X") => 1,
            ("B", "Y") => 2 + 3,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 2,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 1 + 6,
            _ => panic!("Illegal pair"),
        };
        total + score
    });

    println!("## Part 2");
    println!("{}", total);
}
