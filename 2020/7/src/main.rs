use regex::Regex;
use std::{collections::HashSet, env, fs, path::Path};

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
    let mut result_bags: HashSet<String> = HashSet::new();

    find_contained_in("shiny gold", &lines, &mut result_bags);

    println!("## Part 1");
    //result_bags.iter().for_each(|l| println!("{}", l));
    println!("{}", result_bags.len());
}

fn part2(lines: &Vec<&str>) {
    println!("## Part 2");
    println!("{}", count_bags_in("shiny gold", &lines));
}

fn count_bags_in(bag: &str, lines: &Vec<&str>) -> u16 {
    let pattern = bag.to_string() + " bags contain (\\d+.+).";
    let regex = Regex::new(&(pattern)).unwrap();

    lines
        .iter()
        .flat_map(|l| regex.captures(&l))
        .flat_map(|captures| captures.get(1))
        .map(|m| m.as_str())
        .map(|inside| split_and_count(inside, lines))
        .sum()
}

fn split_and_count(inside: &str, lines: &Vec<&str>) -> u16 {
    let pattern = "(\\d+) (.+) bags?";
    let regex = Regex::new(&(pattern)).unwrap();

    let sum = inside
        .split(",")
        .flat_map(|part| regex.captures(part))
        .map(|captures| {
            let mut count = captures
                .get(1)
                .map(|m| m.as_str().parse::<u16>().unwrap_or(0))
                .unwrap_or(0);
            if count > 0 {
                count = count + count * count_bags_in(captures.get(2).unwrap().as_str(), lines);
            }
            count
        })
        .sum();
    sum
}

fn find_contained_in(bag: &str, lines: &Vec<&str>, result_bags: &mut HashSet<String>) {
    let pattern = "\\d+ ".to_string() + bag;

    let regex = Regex::new(&(pattern)).unwrap();

    let is_within: Vec<&str> = lines
        .iter()
        .filter(|l| !l.starts_with(bag))
        .filter(|l| regex.is_match(&l))
        .map(|l| l.to_owned())
        .collect();

    if is_within.len() > 0 {
        for l in is_within {
            let bag = l.split(" bags contain").next().unwrap();

            result_bags.insert(bag.to_owned());
            find_contained_in(&bag, &lines, result_bags);
        }
    }
}
