use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let bits: Vec<Vec<i64>> = data
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split("")
                .filter(|bit| !bit.is_empty())
                .map(|bit| bit.parse().unwrap())
                .collect()
        })
        .collect();
    let bits = transpose(bits);

    part1(&bits);
    part2(&bits);
}

fn part1(bits: &Vec<Vec<i64>>) {
    let len = bits.len();
    let breakeven = bits[0].len() / 2;

    let (gamma, epsilon) =
        bits.iter()
            .enumerate()
            .fold((0, 0), |(mut least, mut most), (idx, vertical)| {
                let zeros = vertical.iter().filter(|b| **b == 0).count();

                if zeros == breakeven {
                    panic!("No most common?");
                }
                if zeros > breakeven {
                    least = least + (1 << (len - idx - 1));
                    (least, most)
                } else {
                    most = most + (1 << (len - idx - 1));
                    (least, most)
                }
            });

    println!("## Part 1");
    println!(
        "gamma: {}, epsilon: {}, power: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn part2(_bits: &Vec<Vec<i64>>) {}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
