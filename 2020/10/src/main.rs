use std::{collections::HashMap, env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut lines: Vec<i64> = data.split("\n").map(|l| l.parse().unwrap()).collect();

    part1(&mut lines);
    part2(&mut lines);
}

fn part1(lines: &mut Vec<i64>) {
    let mut d1 = 0;
    let mut d3 = 1; // own adapter

    lines.sort();
    lines.iter().fold(0, |previous, v| {
        let diff = v - previous;
        match diff {
            1 => d1 = d1 + 1,
            3 => d3 = d3 + 1,
            2 => (),
            _ => panic!("Whoot?"),
        };
        *v
    });

    println!("## Part 1");
    println!("{}*{}={}", d1, d3, d1 * d3);
}

fn part2(lines: &mut Vec<i64>) {
    println!("## Part 2");

    lines.push(0);
    lines.sort();
    lines.push(lines.last().unwrap() + 3);

    let slice = lines.as_slice();

    let mut visited = HashMap::new();
    println!("{}", find_combis(0, slice, &mut visited));
}

fn find_combis(idx: usize, slice: &[i64], visited: &mut HashMap<usize, i64>) -> i64 {
    if idx == slice.len() - 1 {
        return 1;
    }
    if visited.contains_key(&idx) {
        return *visited.get(&idx).unwrap();
    }

    let sum = (idx + 1..slice.len())
        .filter(|next| slice[*next] - slice[idx] <= 3)
        .map(|next| find_combis(next, slice, visited))
        .sum::<i64>();
    visited.insert(idx, sum);
    sum
}
