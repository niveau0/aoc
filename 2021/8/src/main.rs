use std::{env, fs, path::Path};

#[derive(Debug, Clone)]
struct Pattern(String);

#[derive(Debug, Clone)]
struct FourDigitDisplay(Vec<Pattern>);

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");

    let input: Vec<(Vec<Pattern>, FourDigitDisplay)> = data
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.split("|"))
        .map(|mut s| (s.next().unwrap().trim(), s.next().unwrap().trim()))
        .map(|(l, r)| {
            let patterns = l.split(" ").map(|p| Pattern(p.to_owned())).collect();
            let display = FourDigitDisplay(r.split(" ").map(|p| Pattern(p.to_owned())).collect());
            (patterns, display)
        })
        .collect();

    part1(input.clone());
    part2(input.clone());
}

fn part1(input: Vec<(Vec<Pattern>, FourDigitDisplay)>) {
    let sum: u64 = input
        .iter()
        .map(|(patterns, display)| {
            let sum = patterns
                .iter()
                .map(|p| match p.0.len() {
                    2 | 3 | 4 | 7 => display
                        .0
                        .iter()
                        .filter(|dp| p.0.len() == dp.0.len())
                        .map(|_| 1)
                        .sum(),
                    _ => 0,
                })
                .sum::<u64>();
            sum
        })
        .sum();
    println!("## Part 1");
    println!("Sum {}", sum);
}

fn pattern_to_code(p: &Pattern) -> u64 {
    p.0.split("")
        .filter(|l| !l.is_empty())
        .map(|l| match l {
            "a" => 2,
            "b" => 3,
            "c" => 5,
            "d" => 7,
            "e" => 11,
            "f" => 13,
            "g" => 17,
            l => panic!("Unknown letter {}", l),
        })
        .fold(1, |a, v| a * v)
}

fn part2(input: Vec<(Vec<Pattern>, FourDigitDisplay)>) {
    let sum: u64 = input
        .iter()
        .map(|(patterns, display)| {
            let mut digit_codes = vec![0; 10];

            // resolve obvious 1,4,7,8
            patterns.iter().for_each(|p| match p.0.len() {
                2 => {
                    digit_codes[1] = pattern_to_code(p);
                }
                3 => {
                    digit_codes[7] = pattern_to_code(p);
                }
                4 => {
                    digit_codes[4] = pattern_to_code(p);
                }
                7 => {
                    digit_codes[8] = pattern_to_code(p);
                }
                _ => (),
            });

            // resolve 9 before 5
            patterns.iter().for_each(|p| match p.0.len() {
                6 => {
                    let code = pattern_to_code(p);
                    if no_remainder(code, digit_codes[4]) {
                        digit_codes[9] = code;
                    } else if no_remainder(code, digit_codes[1]) {
                        digit_codes[0] = code;
                    } else {
                        digit_codes[6] = code;
                    }
                }
                _ => (),
            });

            // resolve 5 from 9
            patterns.iter().for_each(|p| match p.0.len() {
                5 => {
                    let code = pattern_to_code(p);
                    if no_remainder(code, digit_codes[7]) {
                        digit_codes[3] = code;
                    } else if no_remainder(digit_codes[9], code) {
                        digit_codes[5] = code;
                    } else {
                        digit_codes[2] = code;
                    }
                }
                _ => (),
            });

            let s = display
                .0
                .iter()
                .enumerate()
                .map(|(idx, p)| calc_display(&digit_codes, p, idx as u32))
                .sum::<u64>();
            // dbg!(&num_2_patterncode, &s);
            s
        })
        .sum();

    println!("## Part 2");
    println!("Sum {}", sum);
}

fn no_remainder(value: u64, divisor: u64) -> bool {
    (value as f64) % (divisor as f64) == 0.0
}

fn calc_display(digit_codes: &Vec<u64>, pattern: &Pattern, idx: u32) -> u64 {
    let factor: u64 = 10_u32.pow(3_u32 - idx) as u64;
    digit_codes
        .iter()
        .enumerate()
        .find(|(_, num)| pattern_to_code(pattern) == **num)
        .map(|(idx, _)| idx)
        .unwrap() as u64
        * factor
}
