use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut lines: Vec<i64> = data.split("\n").map(|l| l.parse().unwrap()).collect();

    part1(&mut lines);
    part2(&mut lines);
}

fn part1(lines: &mut Vec<i64>) {
    let mut d1 = 0;
    let mut d3 = 1; // own adapter

    lines.sort();
    lines.iter().fold(0, |previous, v| {
        let diff = v - previous;
        match diff {
            1 => d1 = d1 + 1,
            3 => d3 = d3 + 1,
            2 => (),
            _ => panic!("Whoot?"),
        };
        *v
    });

    println!("## Part 1");
    println!("{}*{}={}", d1, d3, d1 * d3);
}

fn part2(lines: &mut Vec<i64>) {
    lines.sort();
    lines.push(lines.last().unwrap() + 3);
    let slice = lines.as_slice();

    println!("## Part 2");
    println!(
        "{}",
        (1..4)
            .map(|distance| {
                let mut sum = 0;
                for idx in 0..slice.len() {
                    if distance < slice[idx] {
                        break;
                    }
                    sum = sum + find_streak(0, distance, &slice[idx..])
                }
                sum
            })
            .sum::<i64>()
    );
}

fn find_streak(value: i64, distance: i64, slice: &[i64]) -> i64 {
    let next = slice[0];
    if value + distance != next {
        return 0;
    }
    if slice.len() == 1 {
        return 1;
    }

    (1..4)
        .map(|distance| {
            let mut sum = 0;
            for idx in 1..slice.len() {
                if next + distance < slice[idx] {
                    break;
                }
                sum = sum + find_streak(next, distance, &slice[idx..])
            }
            sum
        })
        .sum::<i64>()
}
