use std::{env, fs, path::Path};
fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let initial: Vec<u128> = data
        .split(",")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();
    let mut fishes: Vec<u128> = vec![0; 9];
    initial
        .iter()
        .for_each(|state| fishes[*state as usize] += 1);
    part1(fishes.clone(), 80);
    part2(fishes.clone(), 256);
}

fn part1(mut fishes: Vec<u128>, days: usize) {
    (0..days).for_each(|_| {
        run(&mut fishes);
    });

    println!("## Part 1");
    let sum: u128 = fishes.iter().sum();
    println!("Sum: {}", sum);
}

fn part2(mut fishes: Vec<u128>, days: usize) {
    (0..days).for_each(|_| {
        run(&mut fishes);
    });

    println!("## Part 2");
    let sum: u128 = fishes.iter().sum();
    println!("Sum: {}", sum);
}

fn run(fishes: &mut Vec<u128>) {
    let pregnant = fishes[0].clone();

    (1..fishes.len()).for_each(|idx| fishes[idx - 1] = fishes[idx]);
    fishes[8] = pregnant;
    fishes[6] += pregnant;
}
