use core::panic;
use std::{collections::HashMap, env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<(Vec<char>, Vec<u8>)> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(' ').unwrap())
        .map(|(l, r)| {
            (
                l.chars().collect(),
                r.split(',')
                    .map(|d| d.parse::<u8>().unwrap())
                    .collect::<Vec<u8>>(),
            )
        })
        .collect();

    println!("## Part 1");
    part1(&mut data.clone()[..]);

    println!("## Part 2");
    part2(&mut data.clone()[..]);
}

fn part1(data: &mut [(Vec<char>, Vec<u8>)]) {
    println!(
        "{}",
        data.iter_mut()
            .map(|(record, groups)| {
                find_arrangements(
                    &mut record[..],
                    &mut groups[..],
                    &GroupType::Ok,
                    &mut HashMap::new(),
                )
            })
            .sum::<u64>()
    );
}

fn part2(data: &mut [(Vec<char>, Vec<u8>)]) {
    println!(
        "{}",
        data.iter_mut()
            .map(|(record, groups)| {
                let mut new_record = record.clone();
                let mut new_groups = groups.clone();
                (1..=4).for_each(|_| {
                    new_record.push('?');
                    new_record.extend(record.iter());
                    new_groups.extend(groups.iter());
                });
                find_arrangements(
                    &mut new_record[..],
                    &mut new_groups[..],
                    &GroupType::Ok,
                    &mut HashMap::new(),
                )
            })
            .sum::<u64>()
    );
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct CacheEntry {
    record: Vec<char>,
    groups: Vec<u8>,
    previous_type: GroupType,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum GroupType {
    Ok,
    Damaged,
}

fn find_arrangements(
    record: &mut [char],
    groups: &mut [u8],
    previous_type: &GroupType,
    cache: &mut HashMap<CacheEntry, u64>,
) -> u64 {
    if record.is_empty() {
        return if groups.is_empty() || (groups.len() == 1 && groups[0] == 0) {
            1
        } else {
            0
        };
    }

    let cache_entry = CacheEntry {
        record: record.to_vec(),
        groups: groups.to_vec(),
        previous_type: *previous_type,
    };

    let r = if let Some(v) = cache.get(&cache_entry) {
        *v
    } else {
        match (record[0], previous_type) {
            ('.', GroupType::Ok) => {
                find_arrangements(&mut record[1..], groups, &GroupType::Ok, cache)
            }
            ('.', GroupType::Damaged) => {
                if groups[0] == 0 {
                    find_arrangements(&mut record[1..], &mut groups[1..], &GroupType::Ok, cache)
                } else {
                    0
                }
            }
            ('#', _) => {
                if groups.is_empty() || groups[0] == 0 {
                    0
                } else {
                    groups[0] -= 1;
                    find_arrangements(&mut record[1..], groups, &GroupType::Damaged, cache)
                }
            }
            ('?', t) => {
                record[0] = '.';
                let a =
                    find_arrangements(&mut record.to_vec()[..], &mut groups.to_vec()[..], t, cache);

                record[0] = '#';
                let b =
                    find_arrangements(&mut record.to_vec()[..], &mut groups.to_vec()[..], t, cache);

                a + b
            }
            _ => panic!(),
        }
    };
    cache.insert(cache_entry, r);
    r
}
