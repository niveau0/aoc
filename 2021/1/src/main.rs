use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut lines: Vec<i64> = data
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();

    part1(&mut lines);
    part2(&mut lines);
}

fn part1(lines: &mut Vec<i64>) {
    let mut count = 0;

    lines.iter().reduce(|previous, v| {
        if v - previous > 0 {
            count = count + 1;
        }
        v
    });

    println!("## Part 1");
    println!("{}", count);
}

fn part2(lines: &mut Vec<i64>) {
    let mut count = 0;

    let sums: Vec<i64> = lines.windows(3).map(|w| w[0] + w[1] + w[2]).collect();
    sums.iter().reduce(|previous, v| {
        if v - previous > 0 {
            count = count + 1;
        }
        v
    });

    println!("## Part 2");
    println!("{}", count);
}
