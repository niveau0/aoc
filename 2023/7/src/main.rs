use std::{cmp::Ordering, collections::HashMap, env, fmt::Display, fs, path::Path};
use itertools::Itertools;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Card(char);

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Card {
    fn value(&self, wildcard: bool) -> u8 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' if wildcard => 1,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => panic!("Unknown card"),
        }
    }
}

#[derive(Debug, Clone)]
struct Hand(Vec<Card>, u64);

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().join(""))
    }
}

impl Hand {
    fn handtype(&self, wildcard: bool) -> HandType {
        let mut count = HashMap::new();
        for card in &self.0 {
            count.entry(card).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut len: usize = count.len();

        let num_wildcards = if wildcard {
            *count.get(&Card('J')).unwrap_or(&0)
        } else {
            0
        };
        if num_wildcards > 0 {
            len = len.checked_sub(1).unwrap_or(0);
        }

        match len {
            5 => HandType::High,
            4 => HandType::One,
            3 => {
                let max = count.iter().map(|(_, i)| i).max().unwrap() + num_wildcards;
                if max >= 3 {
                    HandType::Three
                } else {
                    HandType::Two
                }
            }
            2 => {
                let max = count.iter().map(|(_, i)| i).max().unwrap() + num_wildcards;
                if max >= 4 {
                    HandType::Four
                } else {
                    HandType::FullHouse
                }
            }
            _ => HandType::Five,
        }
    }
}

#[derive(Debug)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    Two,
    One,
    High,
}
impl HandType {
    fn value(&self) -> u8 {
        match self {
            HandType::Five => 7,
            HandType::Four => 6,
            HandType::FullHouse => 5,
            HandType::Three => 4,
            HandType::Two => 3,
            HandType::One => 2,
            HandType::High => 1,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let hands: Vec<Hand> = data
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| l.split_once(' ').unwrap())
        .map(|(c, b)| Hand(c.chars().map(|c| Card(c)).collect(), b.parse().unwrap()))
        .collect();

    println!("## Part 1");
    part1(&mut hands.clone());

    println!("## Part 2");
    part2(&mut hands.clone());
}

fn sort_hands(hands: &mut [Hand], wildcard: bool) {
    hands.sort_by(|a, b| {
        match a
            .handtype(wildcard)
            .value()
            .cmp(&b.handtype(wildcard).value())
        {
            Ordering::Equal => {
                a.0.iter()
                    .zip(&b.0)
                    .find_map(|(a, b)| {
                        let o = a.value(wildcard).cmp(&b.value(wildcard));
                        if o != Ordering::Equal {
                            Some(o)
                        } else {
                            None
                        }
                    })
                    .unwrap_or(Ordering::Equal)
            }
            o => o,
        }
    });
}

fn part1(hands: &mut [Hand]) {
    sort_hands(hands, false);

    let sum = hands
        .iter()
        .enumerate()
        .map(|(i, c)| (i + 1) as u64 * c.1)
        .sum::<u64>();
    println!("Part1: {:?}", sum);
}

fn part2(hands: &mut [Hand]) {
    sort_hands(hands, true);

    let sum = hands
        .iter()
        .enumerate()
        .map(|(i, c)| (i + 1) as u64 * c.1)
        .sum::<u64>();

    // println!(
    //     "Part2: {:?}",
    //     hands
    //         .iter()
    //         .map(|h| format!("{} ({:?})", h, h.handtype(true)))
    //         .join(",")
    // );
    println!("Part2: {:?}", sum);
}
