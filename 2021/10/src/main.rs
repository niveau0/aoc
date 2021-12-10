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
            stack.clear();
            s.split("")
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
                .unwrap_or(0)
        })
        .sum();

    println!("## Part 1");
    println!("Result {}", result);
}

fn part2(input: Vec<&str>) {
    #[derive(PartialEq)]
    enum State {
        Valid,
        Corrupt,
    }
    let mut stack: Vec<&str> = Vec::with_capacity(200);
    let mut result: Vec<i64> = input
        .into_iter()
        .map(|s| {
            stack.clear();
            let state = s
                .split("")
                .filter(|t| !t.is_empty())
                .map(|t| match t {
                    "(" | "<" | "{" | "[" => {
                        stack.push(t);
                        State::Valid
                    }
                    ")" | ">" | "}" | "]" => {
                        if let Some(top) = stack.pop() {
                            match (top, t) {
                                ("(", ")") | ("[", "]") | ("<", ">") | ("{", "}") => State::Valid,
                                (_, ")") | (_, "]") | (_, "}") | (_, ">") => State::Corrupt,
                                _ => panic!("Unknown token"),
                            }
                        } else {
                            State::Valid
                        }
                    }
                    _ => panic!("Unknown token"),
                })
                .find(|r| *r == State::Corrupt)
                .unwrap_or(State::Valid);
            if state == State::Valid {
                stack
                    .iter()
                    .rev()
                    .map(|t| match *t {
                        "(" => 1,
                        "<" => 4,
                        "{" => 3,
                        "[" => 2,
                        _ => panic!("Unknown token"),
                    })
                    .fold(0, |a, v| a * 5 + v)
            } else {
                0
            }
        })
        .filter(|s| *s > 0)
        .collect();

    if result.len() % 2 == 0 {
        panic!("No middle score?")
    }
    result.sort();
    println!("## Part 2");
    println!("Result {}", result[result.len() / 2]);
}
