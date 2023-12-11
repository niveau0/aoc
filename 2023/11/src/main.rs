use std::{
    collections::HashMap,
    env, fs,
    path::Path,
};

#[derive(Debug)]
struct Universe {
    empty_cols: Vec<usize>,
    empty_rows: Vec<usize>,
    galaxies: HashMap<u32, (usize, usize)>,
}

impl Universe {
    fn new(rows: Vec<String>) -> Self {
        let h = rows.len();
        let w = rows[0].len();
        let mut empty_rows = Vec::new();
        let mut empty_cols = Vec::new();
        let mut galaxies: HashMap<u32, (usize, usize)> = HashMap::new();

        for (i, r) in rows.iter().enumerate() {
            if !r.contains('#') {
                empty_rows.push(i);
            }
        }
        let mut g = 0;
        for c in 0..w {
            let mut has_galaxy: bool = false;
            for r in 0..h {
                let Some(v) = rows[r].chars().nth(c) else {
                    break;
                };
                if v == '#' {
                    has_galaxy = true;
                    g += 1;
                    galaxies.insert(g, (c, r));
                }
            }
            if !has_galaxy {
                empty_cols.push(c);
            }
        }

        Universe {
            empty_cols,
            empty_rows,
            galaxies,
        }
    }

    fn extra_horizontal_space(&self, a: usize, b: usize, value: usize) -> usize {
        self.empty_cols.iter().filter(|c| {
            (a < b && **c > a && **c < b) || (**c > b && **c < a)
        }).map(|_| 1).sum::<usize>() * value
    }

    fn extra_vertical_space(&self, a: usize, b: usize, value: usize) -> usize {
        self.empty_rows.iter().filter(|c| {
            (a < b && **c > a && **c < b) || (**c > b && **c < a)
        }).map(|_| 1).sum::<usize>() * value
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut data: Universe = Universe::new(
        data.split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| l.to_owned())
            .collect(),
    );

    println!("## Part 1");
    part1(&mut data);

    println!("## Part 2");
    part2(&mut data);
}

fn part1(data: &mut Universe) {
    println!("{}", sum_distance(data, 1));
}

fn part2(data: &mut Universe) {
    println!("{}", sum_distance(data, 1000000 - 1)); // replace 1 with 1000000
}

fn sum_distance(data: &mut Universe, expanse_value: usize) -> usize {
    let mut sum = 0;
    for a in 1..=data.galaxies.len() - 1 {
        for b in a + 1..=data.galaxies.len() {
            let p1 = data.galaxies.get(&(a as u32)).unwrap();
            let p2 = data.galaxies.get(&(b as u32)).unwrap();
            let dist = p1.0.abs_diff(p2.0)
                + p1.1.abs_diff(p2.1)
                + data.extra_horizontal_space(p1.0, p2.0, expanse_value)
                + data.extra_vertical_space(p1.1, p2.1, expanse_value);
            sum += dist;
        }
    }
    sum
}
