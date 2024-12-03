use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    // let data: Vec<&str> = data.split('\n').filter(|l| !l.is_empty()).collect();

    println!("## Part 1");
    part1(&data);

    println!("## Part 2");
    part2(&data);
}

fn part1(data: &str) {
    let e = regex::Regex::new(r#"(mul\(\s*(\d+)\s*,\s*(\d+)\s*\))"#).unwrap();
    let mut sum = 0;
    for (_, [_m, a, b]) in e.captures_iter(data).map(|c| c.extract()) {
        //println!("{} {} {}", m, a, b)
        sum += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
    }
    println!("{}", sum);
}

fn part2(data: &str) {
    let mut sum = 0;
    let e = regex::Regex::new(r#"(mul\(\s*(\d+)\s*,\s*(\d+)\s*\))"#).unwrap();
    let mut do_idx = 0;

    loop {
        let dont_idx = data[do_idx..]
            .find("don't()")
            .map(|i| i + do_idx)
            .unwrap_or(data.len());

        for (_, [_m, a, b]) in e
            .captures_iter(&data[do_idx..dont_idx])
            .map(|c| c.extract())
        {
            sum += a.parse::<i64>().unwrap() * b.parse::<i64>().unwrap();
        }
        if dont_idx >= data.len() {
            break;
        }
        let Some(idx) = data[dont_idx..].find("do()") else {
            break;
        };
        do_idx = idx + dont_idx;
    }
    println!("{}", sum);
}
