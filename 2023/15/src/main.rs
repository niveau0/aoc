use std::{collections::HashMap, env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<&str> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .flat_map(|l| l.split(',').map(|s| s.trim()))
        .collect();

    println!("## Part 1");
    part1(&data);

    println!("## Part 2");
    part2(&data);
}

fn part1(data: &Vec<&str>) {
    let sum = data.iter().map(|s| hash(s)).map(|h| h as u64).sum::<u64>();
    println!("{}", sum);
}

fn hash(s: &str) -> u16 {
    s.as_bytes().iter().fold(0, |mut acc, v| {
        acc += *v as u16;
        acc *= 17;
        acc % 256
    })
}

fn part2(data: &Vec<&str>) {
    let mut boxes: HashMap<u16, Vec<(&str, &str)>> = HashMap::new();

    data.iter().for_each(|s| {
        let (label, lens, op) = if let Some((label, lens)) = s.split_once('=') {
            (label, lens, '=')
        } else if let Some((label, lens)) = s.split_once('-') {
            (label, lens, '-')
        } else {
            panic!()
        };
        let h = hash(label);

        match op {
            '=' => {
                boxes
                    .entry(h)
                    .and_modify(|e| {
                        if let Some(pos) = e.iter().position(|(la, _)| *la == label) {
                            e[pos] = (label, lens);
                        } else {
                            e.push((label, lens));
                        }
                    })
                    .or_insert(vec![(label, lens)]);
            }
            '-' => {
                boxes.entry(h).and_modify(|e| {
                    if let Some(pos) = e.iter().position(|(la, _)| *la == label) {
                        e.remove(pos);
                    }
                });
            }
            _ => panic!(),
        }
    });

    let sum = boxes
        .iter()
        .map(|(b, content)| {
            content
                .iter()
                .enumerate()
                .fold(0, |acc, (slot, (_, lens))| {
                    acc + (b + 1) as u64 * (slot + 1) as u64 * lens.parse::<u64>().unwrap()
                })
        })
        .sum::<u64>();
    println!("{}", sum);
}
