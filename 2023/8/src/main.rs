use std::{
    collections::HashMap,
    env, fs,
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");

    let mut lines = data.split('\n').filter(|l| !l.is_empty());
    let instructions = lines.next().unwrap();
    let network: HashMap<String, (String, String)> = lines
        .map(|l| l.split_once('=').unwrap())
        .map(|(l, r)| (l.trim(), r.trim().split_once(", ").unwrap()))
        .map(|(g, (l, r))| {
            (
                g.to_owned(),
                (l[1..].to_owned(), r[..r.len() - 1].to_owned()),
            )
        })
        .collect();

    println!("## Part 1");
    part1(&instructions, &network);

    println!("## Part 2");
    part2(&instructions, &network);
}

fn part1(instructions: &str, network: &HashMap<String, (String, String)>) {
    let mut steps = 0;
    let mut place = "AAA";
    loop {
        for i in instructions.chars() {
            let (l, r) = network.get(place).unwrap();
            steps += 1;
            if i == 'L' {
                place = l;
            } else {
                place = r;
            }
            // dbg!(l, r, place);
        }
        if place == "ZZZ" {
            break;
        }
    }
    println!("{}", steps);
}

fn part2(instructions: &str, network: &HashMap<String, (String, String)>) {
    let places: Vec<&String> = network
        .iter()
        .map(|(n, _)| n)
        .filter(|n| n.ends_with("A"))
        .collect();

    let steps: Vec<u128> = places
        .iter()
        .map(|p| steps_till_end(p, instructions, network))
        .collect();
    println!("{}", steps.into_iter().reduce(|a,b| num::integer::lcm(a, b)).unwrap());
}

fn steps_till_end(
    place: &String,
    instructions: &str,
    network: &HashMap<String, (String, String)>,
) -> u128 {
    let mut place = place;
    let mut steps = 0;
    loop {
        for i in instructions.chars() {
            let (l, r) = network.get(place).unwrap();
            steps += 1;
            if i == 'L' {
                place = l;
            } else {
                place = r;
            }
        }
        if place.ends_with("Z") {
            dbg!(place);
            break;
        }
    }
    steps
}
