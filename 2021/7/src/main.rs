use std::{env, fs, path::Path};
fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let initial: Vec<i64> = data
        .split(",")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect();
    let mut crabs: Vec<i64> = vec![0; 2000];
    initial.iter().for_each(|state| crabs[*state as usize] += 1);
    part1(crabs.clone());
    part2(crabs.clone());
}

fn part1(crabs: Vec<i64>) {
    let fuel = (0..2000)
        .map(|pos| {
            crabs
                .iter()
                .enumerate()
                .filter(|(_, v)| **v != 0)
                .fold(0, |fuel, (idx, v)| {
                    fuel + (idx as i64 - pos as i64).abs() * v
                })
        })
        .min()
        .unwrap();
    println!("## Part 1");
    println!("Fuel {}", fuel);
}

fn part2(crabs: Vec<i64>) {
    let fuel = (0..2000)
        .map(|pos| {
            crabs
                .iter()
                .enumerate()
                .filter(|(_, v)| **v != 0)
                .fold(0, |fuel, (idx, v)| {
                    let dist = (idx as i64 - pos as i64).abs();
                    if dist > 0 {
                        let sum = dist * (dist + 1) / 2;
                        fuel + sum * v
                    } else {
                        fuel
                    }
                })
        })
        .min()
        .unwrap();
    println!("## Part 2");
    println!("Fuel {}", fuel);
}
