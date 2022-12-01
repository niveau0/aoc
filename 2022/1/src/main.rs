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
        .map(|l| if l.is_empty() { 0 } else { l.parse().unwrap() })
        .collect();

    part1(&mut lines);
    part2(&mut lines);
}

fn part1(calories: &mut Vec<i64>) {
    let mut max = 0;
    let mut sum = 0;
    for c in calories {
        if *c != 0 {
            sum += *c;
        } else if sum > max {
            max = sum;
            sum = 0;
        } else {
            sum = 0;
        }
    }

    println!("## Part 1");
    println!("{}", max);
}

fn part2(calories: &mut Vec<i64>) {
    let mut sum = 0;
    let mut sums = vec![];
    for c in calories {
        if *c != 0 {
            sum += *c;
        } else {
            sums.push(sum);
            sum = 0;
        }
    }

    sums.sort();
    sums.reverse();
    let total: i64 = sums.into_iter().take(3).sum();
    println!("## Part 1");
    println!("{}", total);
}
