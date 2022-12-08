use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<Vec<char>> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let w = data[0].len();
    let h = data.len();

    let outer = w * 2 + (h - 2) * 2;
    let mut seen = 0;
    for row in 1..h - 1 {
        for col in 1..w - 1 {
            if !invisible(row, col, &data) {
                seen += 1;
            }
        }
    }
    println!("## Part 1");
    println!("outer: {}, inner: {}, total: {}", outer, seen, outer + seen);

    let mut scores = vec![];
    for row in 1..h - 1 {
        for col in 1..w - 1 {
            scores.push(score(row, col, &data));
        }
    }
    scores.sort();
    scores.reverse();
    println!("## Part 2");
    println!("best score: {}", scores[0]);
}

fn invisible(row: usize, col: usize, data: &Vec<Vec<char>>) -> bool {
    let w = data[0].len();
    let h = data.len();
    let tree = data[row][col];
    (0..col)
        .rev()
        .find(|c| data[row][*c] >= tree)
        .map(|_| true)
        .unwrap_or(false)
        && (col + 1..w)
            .find(|c| data[row][*c] >= tree)
            .map(|_| true)
            .unwrap_or(false)
        && (0..row)
            .rev()
            .find(|r| data[*r][col] >= tree)
            .map(|_| true)
            .unwrap_or(false)
        && (row + 1..h)
            .find(|r| data[*r][col] >= tree)
            .map(|_| true)
            .unwrap_or(false)
}

fn score(row: usize, col: usize, data: &Vec<Vec<char>>) -> usize {
    let w = data[0].len();
    let h = data.len();
    let tree = data[row][col];
    (0..col)
        .rev()
        .find(|c| data[row][*c] >= tree)
        .map(|blkidx| col - blkidx)
        .unwrap_or(col)
        * (col + 1..w)
            .find(|c| data[row][*c] >= tree)
            .map(|blkidx| blkidx - col)
            .unwrap_or(w - col - 1)
        * (0..row)
            .rev()
            .find(|r| data[*r][col] >= tree)
            .map(|blkidx| row - blkidx)
            .unwrap_or(row)
        * (row + 1..h)
            .find(|r| data[*r][col] >= tree)
            .map(|blkidx| blkidx - row)
            .unwrap_or(h - row - 1)
}
