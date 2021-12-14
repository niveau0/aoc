use std::{collections::HashMap, env, fs, path::Path};

#[derive(Debug, Clone)]
struct Input(Vec<String>, HashMap<String, String>);

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");

    let mut line_iter = data.split("\n");

    let polymer: Vec<String> = line_iter
        .next()
        .unwrap()
        .split("")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && s.is_ascii())
        .map(|s| s.to_owned())
        .collect();

    let mut transformations = HashMap::new();

    for l in line_iter {
        if l.is_empty() {
            continue;
        }
        let (l, r) = l.split_once(" -> ").unwrap();
        transformations.insert(l.to_owned(), r.to_owned());
    }

    let input = Input(polymer, transformations);
    part1(&input);
    part2(&input);
}

fn part1(input: &Input) {
    let mut input = input.clone();

    (0..10).for_each(|_| {
        input.transform();
    });

    let polymer = input.0;
    println!("## Part 1");
    println!(
        "Result {}",
        polymer.iter().fold("".to_owned(), |a, b| a + b)
    );

    let (most, least, min, max) = find_min_max(polymer);

    println!(
        "Most: {}({}), Least: {}({}), Diff {}",
        most,
        max,
        least,
        min,
        max - min
    );
}

fn part2(input: &Input) {
    let mut input = input.clone();

    (0..40).for_each(|_| {
        input.transform();
    });

    println!("transformed");
    let polymer = input.0;
    println!("## Part 1");
    println!(
        "Result {}",
        polymer.iter().fold("".to_owned(), |a, b| a + b)
    );

    let (most, least, min, max) = find_min_max(polymer);

    println!(
        "Most: {}({}), Least: {}({}), Diff {}",
        most,
        max,
        least,
        min,
        max - min
    );
}

fn find_min_max(mut polymer: Vec<String>) -> (String, String, usize, usize) {
    let mut most = "";
    let mut least = "";
    let mut min = usize::MAX;
    let mut max = 0;
    polymer.sort();
    let mut letters = polymer.clone();
    letters.dedup();
    for letter in &letters {
        let c = polymer.iter().filter(|l| **l == *letter).count();

        if c < min {
            least = &letter;
            min = c;
        }
        if c > max {
            most = &letter;
            max = c;
        }
    }
    (most.to_owned(), least.to_owned(), min, max)
}

impl Input {
    fn transform(&mut self) {
        let polymer = &self.0;
        let mut next = vec![];
        for idx in 0..polymer.len() - 1 {
            let pair = polymer[idx].to_owned() + &polymer[idx + 1];
            let t = self.1.get(&pair);
            if let Some(t) = t {
                next.push(polymer[idx].to_owned());
                next.push(t.to_owned());
            } else {
                next.push(polymer[idx].to_owned());
                next.push(polymer[idx + 1].to_owned());
            }
        }
        next.push(polymer[polymer.len() - 1].to_owned());

        self.0 = next;
    }
}
