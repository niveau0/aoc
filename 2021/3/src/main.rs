use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let matrix: Vec<Vec<i64>> = data
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

    part1(&matrix);
    part2(matrix.clone());
}

fn part1(matrix: &Vec<Vec<i64>>) {
    let transposed = transpose(matrix.clone());
    let len = transposed.len();
    let breakeven = transposed[0].len() / 2;

    let (gamma, epsilon) = transposed.iter().enumerate().fold(
        (0, 0),
        |(mut least, mut most), (idx, vertical_bits)| {
            let zeros = vertical_bits.iter().filter(|b| **b == 0).count();

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
        },
    );

    println!("## Part 1");
    println!(
        "gamma: {}, epsilon: {}, power: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

enum Mode {
    MostCommon,
    LeastCommon,
}

fn find_most_common_line(matrix: &Vec<Vec<i64>>, idx: usize, mode: Mode) -> Vec<i64> {
    let breakeven = matrix.len() / 2;

    let zeros = matrix
        .iter()
        .map(|line_bits| line_bits[idx])
        .filter(|bit| *bit == 0)
        .count();

    let relevant_bit = match mode {
        Mode::MostCommon => {
            if zeros > breakeven {
                0
            } else {
                1
            }
        }
        Mode::LeastCommon => {
            if zeros <= breakeven {
                0
            } else {
                1
            }
        }
    };

    let remaining_matrix: Vec<Vec<i64>> = matrix
        .iter()
        .filter(|line_bits| line_bits[idx] == relevant_bit)
        .map(|vec| vec.to_owned())
        .collect();

    if remaining_matrix.len() == 1 {
        remaining_matrix[0].clone()
    } else {
        find_most_common_line(&remaining_matrix, idx + 1, mode)
    }
}

fn part2(matrix: Vec<Vec<i64>>) {
    let most_common = find_most_common_line(&matrix, 0, Mode::MostCommon);
    let least_common = find_most_common_line(&matrix, 0, Mode::LeastCommon);

    let oxygen = fold_to_decimal(&most_common);
    let co2 = fold_to_decimal(&least_common);
    println!("## Part 2");
    println!("oxygen: {}, co2: {}, rating: {}", oxygen, co2, oxygen * co2);
}

fn fold_to_decimal(bits: &Vec<i64>) -> i32 {
    bits.iter().enumerate().fold(0, |aggr, (idx, v)| {
        if *v == 0 {
            aggr
        } else {
            aggr + (1 << (bits.len() - idx - 1))
        }
    })
}

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
