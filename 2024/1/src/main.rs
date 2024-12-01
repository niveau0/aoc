use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<&str> = data.split('\n').filter(|l| !l.is_empty()).collect();
    let data: (Vec<u64>, Vec<u64>) = data
        .iter()
        .map(|s| s.split_once(" ").unwrap())
        .map(|(a, b)| {
            (
                a.trim().parse::<u64>().unwrap(),
                b.trim().parse::<u64>().unwrap(),
            )
        })
        .collect();

    println!("## Part 1");
    part1(data.clone());

    println!("## Part 2");
    part2(&data);
}

fn part1(mut data: (Vec<u64>, Vec<u64>)) {
    data.0.sort();
    data.1.sort();
    let sum: u64 = data
        .0
        .iter()
        .zip(data.1.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    println!("Part1: {}", sum);
}

fn part2(data: &(Vec<u64>, Vec<u64>)) {
    let sum: u64 = data
        .0
        .iter()
        .map(|a| *a * data.1.iter().filter(|b| *b == a).count() as u64)
        .sum();
    println!("Part2: {}", sum);
}
