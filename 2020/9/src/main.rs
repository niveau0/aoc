use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    let preamble: usize = args
        .get(2)
        .expect("Missing preamble length parameter")
        .parse()
        .unwrap();
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines: Vec<i64> = data.split("\n").map(|l| l.parse().unwrap()).collect();

    part1(&lines, preamble);
    part2(&lines, preamble);
}

fn part1(lines: &Vec<i64>, preamble: usize) {
    let result = (preamble..lines.len())
        .map(|idx| (idx, lines[idx]))
        .find(|(idx, v)| !find_pair(v, &lines[idx - preamble..*idx]))
        .unwrap();

    println!("## Part 1");
    println!("(idx, number):{:?}", result);
}

fn find_pair(v: &i64, slice: &[i64]) -> bool {
    (0..slice.len())
        .find(|i| {
            (0..slice.len())
                .find(|j| i != j && slice[*i] + slice[*j] == *v)
                .map(|_| true)
                .unwrap_or(false)
        })
        .map(|_| true)
        .unwrap_or(false)
}

fn part2(lines: &Vec<i64>, preamble: usize) {
    let (_, target) = (preamble..lines.len())
        .map(|idx| (idx, lines[idx]))
        .find(|(idx, v)| !find_pair(v, &lines[idx - preamble..*idx]))
        .unwrap();

    println!("## Part 2");
    let size = lines.len();
    (0..size).find(|idx| {
        let mut min = std::i64::MAX;
        let mut max = 0;
        (*idx..size).map(|i| lines[i]).fold(0, |s, v| {
            if s == target || v == target {
                v
            } else {
                if v < min {
                    min = v;
                }
                if v > max {
                    max = v;
                }
                if s + v == target {
                    println!("result: {:?}", min + max);
                }
                s + v
            }
        });
        false
    });
}
