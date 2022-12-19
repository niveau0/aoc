use std::{env, fs, path::Path};

use itertools::Itertools;

#[derive(Debug)]
struct Sensor {
    pos: (i32, i32),
    beacon: (i32, i32),
    distance: u32,
}

impl Sensor {
    fn new(pos: (i32, i32), beacon: (i32, i32)) -> Self {
        Sensor {
            pos,
            beacon,
            distance: Sensor::distance(pos, beacon),
        }
    }

    fn distance(p1: (i32, i32), p2: (i32, i32)) -> u32 {
        p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let regex =
        regex::Regex::new("Sensor at x=(.+), y=(.+): closest beacon is at x=(.+), y=(.+)").unwrap();
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<Sensor> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .flat_map(|s| regex.captures(s))
        .map(|captures| {
            Sensor::new(
                (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
            )
        })
        .collect();

    // assuming undefined specification: how big is the map?
    let bounds = boundaries(&data);

    let data: Vec<Sensor> = data
        .into_iter()
        .sorted_by(|s1, s2| s1.distance.cmp(&s2.distance))
        .collect();
    let no_beacons = find_no_beacons(&data, bounds);
    if bounds.1 .1 < 100 {
        print(&data, bounds, &no_beacons);
    }

    println!("## Part 1");
    println!("total: {}", &no_beacons.len());

    let beacon_pos = find_beacon(&data, bounds).unwrap();
    dbg!(&beacon_pos);
    println!("## Part 2");
    println!(
        "total: {}",
        beacon_pos.0 as u128 * 4000000 + beacon_pos.1 as u128
    );
}

fn find_beacon(data: &[Sensor], bounds: ((i32, i32), (i32, i32))) -> Option<(i32, i32)> {
    let max = if bounds.1 .1 > 100 { 4000000 } else { 20 };

    data.iter().find_map(|s| test_outline(data, s, max))
}

fn test_outline(data: &[Sensor], s: &Sensor, max: i32) -> Option<(i32, i32)> {
    (0..=s.distance).find_map(|d| {
        let d = d as i32;
        let left = s.pos.0 - s.distance as i32 - 1;
        let right = s.pos.0 + s.distance as i32 + 1;
        let pts = &[
            (left + d, s.pos.1 - d),
            (right - d, s.pos.1 - d),
            (left + d, s.pos.1 + d),
            (right + d, s.pos.1 + d),
        ];
        pts.into_iter()
            .filter(|(x, y)| *x > 0 && *y > 0 && *x <= max && *y < max)
            .find(|p| {
                !data
                    .iter()
                    .any(|s| Sensor::distance(s.pos, **p) <= s.distance)
            })
            .copied()
    })
}

fn print(data: &[Sensor], bounds: ((i32, i32), (i32, i32)), unreached: &[(i32, i32)]) {
    let width = bounds.1 .0 - bounds.0 .0;
    let height = bounds.1 .1 - bounds.0 .1;
    (0..=height).for_each(|y| {
        (0..=width).for_each(|x| {
            let pos = (x + bounds.0 .0, y + bounds.0 .1);
            if data.iter().any(|s| s.pos == pos) {
                print!("S");
            } else if data.iter().any(|s| s.beacon == pos) {
                print!("B");
            } else if unreached.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        });
        println!();
    });
}

fn find_no_beacons(data: &[Sensor], bounds: ((i32, i32), (i32, i32))) -> Vec<(i32, i32)> {
    let y = if bounds.1 .1 > 100 { 2000000 } else { 10 };

    (bounds.0 .0..bounds.1 .0)
        .filter(|x| {
            // known beacons do not count?
            data.iter()
                .any(|s| Sensor::distance(s.pos, (*x, y)) <= s.distance && s.beacon != (*x, y))
        })
        .map(|x| (x, y))
        .collect()
}

fn boundaries(data: &[Sensor]) -> ((i32, i32), (i32, i32)) {
    let min_x = data.iter().map(|s| s.pos.0 - s.distance as i32).min();
    let max_x = data.iter().map(|s| s.pos.0 + s.distance as i32).max();
    let min_y = data.iter().map(|s| s.pos.1 - s.distance as i32).min();
    let max_y = data.iter().map(|s| s.pos.1 + s.distance as i32).max();
    (
        (min_x.unwrap(), min_y.unwrap()),
        (max_x.unwrap(), max_y.unwrap()),
    )
}
