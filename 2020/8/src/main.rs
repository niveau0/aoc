use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines: Vec<&str> = data.split("\n").collect();

    part1(&lines);
    part2(&lines);
}

fn part1(lines: &Vec<&str>) {
    let mut idx: usize = 0;
    let mut accu = 0;
    let mut visited: Vec<usize> = vec![];

    loop {
        if visited.contains(&idx) {
            break;
        }
        visited.push(idx);
        let mut iter = lines[idx].split(" ").take(2);
        let (opc, param) = (
            iter.next().unwrap(),
            iter.next().unwrap_or("0").parse::<i16>().unwrap(),
        );

        // dbg!(&opc);
        // dbg!(&param);
        match opc {
            "acc" => {
                accu = accu + param;
                idx = idx + 1;
            }
            "jmp" => {
                let i = (idx as i16) + param;
                idx = i as usize;
            }
            _ => idx = idx + 1,
        };
    }

    println!("## Part 1");
    println!("accu: {}", accu);
}

enum Fail {
    InfiniteLoop,
}

fn part2(lines: &Vec<&str>) {
    let mut replace_idx = 0;

    loop {
        if let Ok(accu) = run_program(&lines, replace_idx) {
            println!("## Part 2");
            println!("accu: {}", accu);
            break;
        }
        replace_idx = replace_idx + 1;
    }
}

fn run_program(lines: &&Vec<&str>, replace_idx: usize) -> Result<i16, Fail> {
    let mut idx: usize = 0;
    let mut accu = 0;
    let mut visited: Vec<usize> = vec![];

    loop {
        if idx >= lines.len() {
            return Ok(accu);
        }
        if visited.contains(&idx) {
            return Err(Fail::InfiniteLoop);
        }
        visited.push(idx);
        let mut iter = lines[idx].split(" ").take(2);
        let (opc, param) = (
            iter.next().unwrap(),
            iter.next().unwrap_or("0").parse::<i16>().unwrap(),
        );

        // dbg!(&opc);
        // dbg!(&param);
        match opc {
            "acc" => {
                accu = accu + param;
                idx = idx + 1;
            }
            "jmp" => {
                if replace_idx == idx {
                    idx = idx + 1;
                    dbg!("replace jmp", param);
                } else {
                    let i = (idx as i16) + param;
                    idx = i as usize;
                }
            }
            "nop" => {
                if replace_idx == idx {
                    let i = (idx as i16) + param;
                    idx = i as usize;
                    dbg!("replace nop", param);
                } else {
                    idx = idx + 1;
                }
            }
            _ => panic!("unknown opcode"),
        };
    }
}
