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

fn part1(_data: &[&str]) {
    //println!("Part1: {}", sum);
}

fn part2(_data: &[&str]) {
    //println!("Part2: {}", sum);
}
