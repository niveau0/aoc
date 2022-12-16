use itertools::Itertools;
use serde_json::value::RawValue;
use std::{cmp::Ordering, env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<(&RawValue, &RawValue)> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|s| serde_json::from_str(s).unwrap())
        .tuples()
        .collect();

    println!("## Part 1");
    println!("total: {}", find_index_sum(&data));

    println!("## Part 2");
    let mut all: Vec<&RawValue> = data
        .into_iter()
        .flat_map(|(l, r)| [l, r].into_iter())
        .collect();
    // add dividers
    all.push(serde_json::from_str("[[2]]").unwrap());
    all.push(serde_json::from_str("[[6]]").unwrap());
    all.sort_by(|x, y| compare(x, y));

    //dbg!(all);
    let decoder_key: usize = all
        .iter()
        .enumerate()
        .filter(|(_, v)| v.get() == "[[2]]" || v.get() == "[[6]]")
        .map(|(idx, _)| idx + 1)
        .product();
    println!("total: {}", decoder_key);
}

fn find_index_sum(data: &[(&RawValue, &RawValue)]) -> u16 {
    data.iter()
        .enumerate()
        .fold(0, |aggr, (idx, tuple)| match compare(tuple.0, tuple.1) {
            Ordering::Less => aggr + 1 + idx as u16,
            _ => aggr,
        })
}

fn compare(left: &RawValue, right: &RawValue) -> Ordering {
    let left_vec = serde_json::from_str::<Vec<&RawValue>>(left.get());
    let right_vec = serde_json::from_str::<Vec<&RawValue>>(right.get());

    match (left_vec, right_vec) {
        (Ok(l), Ok(r)) => cmp(&l, &r),
        (Ok(l), _) => cmp(&l, &[right]),
        (_, Ok(r)) => cmp(&[left], &r),
        _ => {
            let l: u16 = serde_json::from_str(left.get()).unwrap();
            let r: u16 = serde_json::from_str(right.get()).unwrap();
            l.cmp(&r)
        }
    }
}

fn cmp(left: &[&RawValue], right: &[&RawValue]) -> Ordering {
    let result = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| compare(l, r))
        .find(|o| *o != Ordering::Equal);
    score(result, left, right)
}

fn score(result: Option<Ordering>, left: &[&RawValue], right: &[&RawValue]) -> Ordering {
    match result {
        None if left.len() > right.len() => Ordering::Greater,
        None if left.len() < right.len() => Ordering::Less,
        Some(order) => order,
        _ => Ordering::Equal,
    }
}
