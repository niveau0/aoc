use std::{env, fs, path::Path};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<&str> = data.split('\n').filter(|l| !l.is_empty()).collect();
    let data = data
        .iter()
        .map(|s| s.split_whitespace())
        .map(|s| s.map(|n| n.trim().parse().unwrap()).collect::<Vec<u64>>())
        .collect::<Vec<Vec<u64>>>();

    println!("## Part 1");
    part1(&data);

    println!("## Part 2");
    part2(&data);
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum ReportType {
    Fail,
    Inc,
    Dec,
}

fn part1(data: &[Vec<u64>]) {
    let sum = data
        .iter()
        .map(analyse_report)
        .map(|result| match result {
            true => 0,
            _ => 1,
        })
        .sum::<u64>();
    println!("Part1: {}", sum);
}

fn part2(data: &[Vec<u64>]) {
    let sum = data
        .iter()
        .map(|report| {
            if !analyse_report(report) {
                return 1;
            }
            for i in 0..report.len() {
                let mut r = report.clone();
                r.remove(i);
                if !analyse_report(&r) {
                    return 1;
                }
            }
            return 0;
        })
        .sum::<u64>();
    println!("Part1: {}", sum); //println!("Part2: {}", sum);
}

fn analyse_report(report: &Vec<u64>) -> bool {
    println!();
    let mut stat = None;
    report.iter().tuple_windows().for_each(|(a, b)| {
        let inc = if a < b { true } else { false };
        stat = Some(match &stat {
            Some(ReportType::Fail) => ReportType::Fail,
            Some(ReportType::Dec) if inc => ReportType::Fail,
            Some(ReportType::Inc) if !inc => ReportType::Fail,
            _ if a.abs_diff(*b) == 0 || a.abs_diff(*b) > 3 => ReportType::Fail,
            None if !inc => ReportType::Dec,
            None if inc => ReportType::Inc,
            Some(stat) => stat.clone(),
            _ => panic!("Uncovered state"),
        });
        println!("{} {} {:?}", a, b, stat);
    });
    stat.map(|s| s == ReportType::Fail).unwrap()
}
