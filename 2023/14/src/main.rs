use itertools::Itertools;
use std::{collections::HashMap, env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let field: Vec<Vec<char>> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect_vec())
        .collect();

    println!("## Part 1");
    part1(&mut field.clone());

    println!("## Part 2");
    part2(&mut field.clone());
}

fn part1(field: &mut [Vec<char>]) {
    print_field(field);

    tilt_ns(field, false);
    print_field(field);

    println!("{}", sum_north(field));
}

fn part2(field: &mut [Vec<char>]) {
    let mut cache = HashMap::new();
    print_field(field);

    let max_runs = 1000000000;
    for i in 0..max_runs {
        tilt_ns(field, false);
        tilt_we(field, false);
        tilt_ns(field, true);
        tilt_we(field, true);
        let entry = field[..].to_vec();
        if let Some(iteration) = cache.get(&entry) {
            let sequence_len = i - iteration;
            if (max_runs - i - 1) % sequence_len == 0 {
                println!("{} {} {}", sequence_len, i, iteration);
                break;
            }
        }
        cache.insert(entry, i);
    }
    // print_field(field);
    println!("{}", sum_north(field));
}

fn sum_north(field: &mut [Vec<char>]) -> usize {
    let sum = field
        .iter()
        .enumerate()
        .map(|(i, r)| (field.len() - i) * r.iter().filter(|c| **c == 'O').count())
        .sum::<usize>();
    sum
}

fn tilt_ns(field: &mut [Vec<char>], reverse: bool) {
    let mut empty_idx = None;
    let mut walk_idx = 0;
    for c in 0..field[0].len() {
        while walk_idx < field.len() {
            let mut r = if reverse {
                field.len() - walk_idx - 1
            } else {
                walk_idx
            };

            let s = field[r][c];
            match (s, empty_idx) {
                ('.', None) => empty_idx = Some(r),
                ('O', Some(idx)) => {
                    field[idx][c] = 'O';
                    field[r][c] = '.';
                    empty_idx = None;
                    r = idx;
                }
                ('#', _) => empty_idx = None,
                _ => (),
            }

            if reverse {
                walk_idx = field.len() - r;
            } else {
                walk_idx = r + 1;
            };
        }
        walk_idx = 0;
        empty_idx = None;
    }
}

fn tilt_we(field: &mut [Vec<char>], reverse: bool) {
    let mut empty_idx = None;
    let mut walk_idx = 0;
    for row in 0..field[0].len() {
        while walk_idx < field.len() {
            let mut c = if reverse {
                field[0].len() - walk_idx - 1
            } else {
                walk_idx
            };

            let s = field[row][c];
            match (s, empty_idx) {
                ('.', None) => empty_idx = Some(c),
                ('O', Some(idx)) => {
                    field[row][idx] = 'O';
                    field[row][c] = '.';
                    empty_idx = None;
                    c = idx;
                }
                ('#', _) => empty_idx = None,
                _ => (),
            }

            if reverse {
                walk_idx = field[0].len() - c;
            } else {
                walk_idx = c + 1;
            };
        }
        walk_idx = 0;
        empty_idx = None;
    }
}

fn print_field(field: &mut [Vec<char>]) {
    for r in field {
        println!("{}", r.iter().join(""));
    }
    println!()
}
