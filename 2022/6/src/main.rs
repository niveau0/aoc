use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");

    part1(&data);

    part2(&data);
}

fn part1(data: &str) {
    let mut idx = 0;
    let result = data.as_bytes().windows(4).find(|w| {
        idx += 1;
        w[0] != w[1] && w[0] != w[2] && w[0] != w[3] && w[1] != w[2] && w[1] != w[3] && w[2] != w[3]
    });
    if result.is_none() {
        panic!("Hilfe!");
    }
    println!("## Part 1");
    println!("{}", idx + 3);
}

fn part2(data: &str) {
    let mut idx = 0;
    let result = data.as_bytes().windows(14).find(|w| {
        idx += 1;

        for i in 0..w.len() {
            for j in 0..w.len() {
                if i != j && w[i] == w[j] {
                    return false;
                }
            }
        }
        true
    });
    if result.is_none() {
        panic!("Hilfe!");
    }
    println!("## Part 2");
    println!("{}", idx + 13);
}
