use std::{env, fs, path::Path};

#[derive(Debug, Clone)]
struct Heatmap(Vec<Vec<(u8, bool)>>);

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");

    let input: Vec<&str> = data
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    part1(input.clone());
    part2(input.clone());
}

fn part1(input: Vec<&str>) {
    let mut stack: Vec<&str> = Vec::with_capacity(200);
    let result: i64 = input
        .into_iter()
        .map(|s| {
            let score = s
                .split("")
                .filter(|t| !t.is_empty())
                .map(|t| match t {
                    "(" | "<" | "{" | "[" => {
                        stack.push(t);
                        None
                    }
                    ")" | ">" | "}" | "]" => {
                        if let Some(top) = stack.pop() {
                            match (top, t) {
                                ("(", ")") | ("[", "]") | ("<", ">") | ("{", "}") => None,
                                (_, ")") => Some(3),
                                (_, "]") => Some(57),
                                (_, "}") => Some(1197),
                                (_, ">") => Some(25137),
                                _ => panic!("Unknown token"),
                            }
                        } else {
                            None
                        }
                    }
                    _ => panic!("Unknown token"),
                })
                .find(|r| r.is_some())
                .unwrap_or(None)
                .unwrap_or(0);
            dbg!(s, score);
            score
        })
        .sum();

    println!("## Part 1");
    println!("Result {}", result);
}

fn part2(_input: Vec<&str>) {
    println!("## Part 2");
    // println!("Result {}", a * b * c);
}
