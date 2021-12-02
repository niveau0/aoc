use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let cmds: Vec<(&str, i64)> = data
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.split(" "))
        .map(|mut split| {
            (
                split.next().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    part1(&mut cmds.clone(), (0, 0));
    part2(&mut cmds.clone(), (0, 0, 0));
}

fn part1(cmds: &mut Vec<(&str, i64)>, start: (i64, i64)) {
    let (mut hpos, mut depth) = start;

    cmds.iter().for_each(|cmd| match cmd {
        ("forward", amount) => hpos = hpos + amount,
        ("down", amount) => depth = depth + amount,
        ("up", amount) => depth = depth - amount,
        (cmd, _) => panic!("Unknown cmd {}", cmd),
    });

    println!("## Part 1");
    println!("x: {}, depth: {}, x*depth: {}", hpos, depth, hpos * depth);
}

fn part2(cmds: &mut Vec<(&str, i64)>, start: (i64, i64, i64)) {
    let (mut hpos, mut depth, mut aim) = start;

    cmds.iter().for_each(|cmd| match cmd {
        ("forward", amount) if aim == 0 => hpos = hpos + amount,
        ("forward", amount) if aim > 0 => {
            hpos = hpos + amount;
            depth = depth + aim * amount;
        }
        ("down", amount) => aim = aim + amount,
        ("up", amount) => aim = aim - amount,
        (cmd, _) => panic!("Unknown cmd {}", cmd),
    });

    println!("## Part 2");
    println!("x: {}, depth: {}, x*depth: {}", hpos, depth, hpos * depth);
}
