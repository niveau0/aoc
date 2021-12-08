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

fn to_num(p: &Pattern) -> u64 {
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
            let mut num_2_patterncode = vec![0; 10];

            patterns.iter().for_each(|p| match p.0.len() {
                2 => {
                    num_2_patterncode[1] = to_num(p);
                }
                3 => {
                    num_2_patterncode[7] = to_num(p);
                }
                4 => {
                    num_2_patterncode[4] = to_num(p);
                }
                7 => {
                    num_2_patterncode[8] = to_num(p);
                }
                _ => (),
            });

            patterns.iter().for_each(|p| match p.0.len() {
                6 => {
                    let code = to_num(p);
                    if (code as f64) % (num_2_patterncode[4] as f64) == 0.0 {
                        num_2_patterncode[9] = code;
                    } else if (code as f64) % (num_2_patterncode[1] as f64) == 0.0 {
                        num_2_patterncode[0] = code;
                    } else {
                        num_2_patterncode[6] = code;
                    }
                }
                _ => (),
            });
            patterns.iter().for_each(|p| match p.0.len() {
                5 => {
                    let code = to_num(p);
                    if (code as f64) % (num_2_patterncode[7] as f64) == 0.0 {
                        num_2_patterncode[3] = code;
                    } else if (num_2_patterncode[9] as f64) % (code as f64) == 0.0 {
                        num_2_patterncode[5] = code;
                    } else {
                        num_2_patterncode[2] = code;
                    }
                }
                _ => (),
            });

            let s = display
                .0
                .iter()
                .enumerate()
                .map(|(idx, p)| resolve_num(&num_2_patterncode, p, idx as u32))
                .sum::<u64>();
            // dbg!(&num_2_patterncode, &s);
            s
        })
        .sum();

    println!("## Part 2");
    println!("Sum {}", sum);
}

fn resolve_num(num_2_patterncode: &Vec<u64>, p: &Pattern, idx: u32) -> u64 {
    let factor: u64 = 10_u32.pow(3_u32 - idx) as u64;
    num_2_patterncode
        .iter()
        .enumerate()
        .find(|(_, num)| to_num(p) == **num)
        .map(|(idx, _)| idx)
        .unwrap() as u64
        * factor
}
