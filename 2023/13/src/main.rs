use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut fields = Vec::new();
    let mut field = Vec::new();
    for l in data.split('\n') {
        if l.is_empty() {
            fields.push(field);
            field = Vec::new();
        } else {
            field.push(l)
        }
    }

    println!("## Part 1");
    part1(&fields);

    println!("## Part 2");
    part2(&fields);
}

fn part1(data: &Vec<Vec<&str>>) {
    let sum = data
        .iter()
        .map(|field: &Vec<&str>| {
            let row = find_row(field, false).unwrap_or(0);
            let column = find_column(field, false).unwrap_or(0);
            if row > 0 && column > 0 {
                panic!();
            }
            column + row * 100
        })
        .sum::<usize>();
    println!("{}", sum);
}

fn part2(data: &Vec<Vec<&str>>) {
    let sum = data
        .iter()
        .map(|field: &Vec<&str>| {
            let row = find_row(field, true).unwrap_or(0);
            let column = find_column(field, true).unwrap_or(0);
            if row > 0 && column > 0 {
                panic!();
            }
            column + row * 100
        })
        .sum::<usize>();
    println!("{}", sum);
}

fn find_row(data: &[&str], correction: bool) -> Option<usize> {
    let height = data.len();
    (0..height - 1)
        .map(|idx| (idx, false))
        .find(|(idx, mut corrected)| {
            let min = (*idx).min((height - idx).checked_sub(2).unwrap_or(0));
            let found = (0..=min)
                .all(|offset| is_mirrow_row(data, idx, offset, correction, &mut corrected));
            if found && (!correction || corrected) {
                true
            } else {
                false
            }
        })
        .map(|(idx, _)| idx + 1)
}

fn is_mirrow_row(
    data: &[&str],
    idx: &usize,
    offset: usize,
    correction: bool,
    corrected: &mut bool,
) -> bool {
    if data[idx - offset] == data[idx + 1 + offset] {
        true
    } else if correction && !*corrected {
        let diffs = data[idx - offset]
            .chars()
            .zip(data[idx + 1 + offset].chars())
            .filter(|(a, b)| a != b)
            .count();
        if diffs == 1 {
            *corrected = true;
            true
        } else {
            false
        }
    } else {
        false
    }
}

fn find_column(data: &[&str], correction: bool) -> Option<usize> {
    let width = data[0].len();
    (0..width)
        .map(|idx| (idx, false))
        .find(|(idx, mut corrected)| {
            let min = (*idx).min((width - idx).checked_sub(2).unwrap_or(0));
            let found = (0..=min)
                .all(|offset| is_mirror_column(data, idx, offset, correction, &mut corrected));
            if found && (!correction || corrected) {
                true
            } else {
                false
            }
        })
        .map(|(idx, _)| idx + 1)
}

fn is_mirror_column(
    data: &[&str],
    idx: &usize,
    offset: usize,
    correction: bool,
    corrected: &mut bool,
) -> bool {
    if data
        .iter()
        .all(|row| row.chars().nth(idx - offset) == row.chars().nth(idx + 1 + offset))
    {
        true
    } else if correction && !*corrected {
        let diffs = data
            .iter()
            .filter(|row| row.chars().nth(idx - offset) != row.chars().nth(idx + 1 + offset))
            .count();
        if diffs == 1 {
            *corrected = true;
            true
        } else {
            false
        }
    } else {
        false
    }
}
