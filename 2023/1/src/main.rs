use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<&str> = data.split('\n').filter(|l| !l.is_empty()).collect();

    println!("## Part 1");
    part1(&data);

    println!("## Part 2");
    part2(&data);
}

fn part1(data: &Vec<&str>) {
    let sum = data
        .iter()
        .map(|l| {
            let first = l
                .chars()
                .find(|c| c.is_numeric())
                .and_then(|c| c.to_digit(10))
                .unwrap();
            let last = l
                .chars()
                .rev()
                .find(|c| c.is_numeric())
                .and_then(|c| c.to_digit(10))
                .unwrap();
            first * 10 + last
        })
        .sum::<u32>();
    println!("Part1: {}", sum);
}

fn part2(data: &Vec<&str>) {
    let sum = data
        .iter()
        .map(|l| {
            let first = l
                .chars()
                .enumerate()
                .find_map(|(i, c)| {
                    if c.is_numeric() {
                        Some(c)
                    } else {
                        is_number_word(&l[i..])
                    }
                })
                .and_then(|c| c.to_digit(10))
                .unwrap();
            let last = l
                .chars()
                .rev()
                .enumerate()
                .find_map(|(i, c)| {
                    if c.is_numeric() {
                        Some(c)
                    } else {
                        is_number_word(&l[l.len() - i - 1..])
                    }
                })
                .and_then(|c| c.to_digit(10))
                .unwrap();
            first * 10 + last
        })
        .sum::<u32>();
    println!("Part2: {}", sum);
}

fn is_number_word(l: &str) -> Option<char> {
    match l {
        _ if l.starts_with("zero") => Some('0'),
        _ if l.starts_with("one") => Some('1'),
        _ if l.starts_with("two") => Some('2'),
        _ if l.starts_with("three") => Some('3'),
        _ if l.starts_with("four") => Some('4'),
        _ if l.starts_with("five") => Some('5'),
        _ if l.starts_with("six") => Some('6'),
        _ if l.starts_with("seven") => Some('7'),
        _ if l.starts_with("eight") => Some('8'),
        _ if l.starts_with("nine") => Some('9'),
        _ => None,
    }
}
