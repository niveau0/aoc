use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<Vec<i64>> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect();

    println!("## Part 1");
    part1(&data);

    println!("## Part 2");
    part2(&data);
}

fn part1(data: &[Vec<i64>]) {
    println!(
        "{}",
        data.iter()
            .map(|nums| { nums.last().unwrap() + extrapolate(nums) })
            .sum::<i64>()
    );
}

fn part2(data: &[Vec<i64>]) {
    println!(
        "{}",
        data.iter()
            .map(|nums| {
                let mut nums = nums.to_owned();
                nums.reverse();
                nums
            })
            .map(|nums| { nums.last().unwrap() + extrapolate(&nums) })
            .sum::<i64>()
    );
}

fn extrapolate(nums: &[i64]) -> i64 {
    let diffs: Vec<i64> = nums.windows(2).map(|w| w[1] - w[0]).collect();
    if diffs.iter().all(|n| *n == 0) {
        return 0;
    }
    diffs.last().unwrap() + extrapolate(&diffs)
}
