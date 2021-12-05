use std::{collections::HashMap, env, fs, path::Path};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point2D(i64, i64);

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let vectors: Vec<(Point2D, Point2D)> = data
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            let mut lr = l.split(" -> ");
            let mut pos = lr.next().unwrap().trim().split(",");

            let start = Point2D(
                pos.next().unwrap().trim().parse().unwrap(),
                pos.next().unwrap().trim().parse().unwrap(),
            );
            let mut pos = lr.next().unwrap().trim().split(",");
            let end = Point2D(
                pos.next().unwrap().trim().parse().unwrap(),
                pos.next().unwrap().trim().parse().unwrap(),
            );
            (start, end)
        })
        .collect();

    part1(&vectors);
    part2(&vectors);
}

fn part1(vectors: &Vec<(Point2D, Point2D)>) {
    let points: Vec<Point2D> = vectors
        .iter()
        .filter(|(p1, p2)| p1.0 == p2.0 || p1.1 == p2.1)
        .map(|(p1, p2)| resolve_vector_points(p1, p2))
        .flatten()
        .collect();
    let mut pointmap: HashMap<Point2D, i64> = HashMap::new();
    points.into_iter().for_each(|p| {
        pointmap.entry(p).and_modify(|v| *v += 1).or_insert(1);
    });

    let sum: i64 = pointmap
        .iter()
        .map(|(_, c)| c)
        .filter(|c| **c > 1)
        .map(|_| 1)
        .sum();
    println!("## Part 1");
    println!("Sum: {}", sum);

    // printfield(9, &pointmap);
}

fn get_range(from: i64, to: i64) -> Box<dyn Iterator<Item = i64>> {
    if from < to {
        Box::new(from..=to)
    } else {
        Box::new((to..=from).rev())
    }
}

// fn printfield(size: i64, pointmap: &HashMap<Point2D, i64>) {
//     (0..=size).for_each(|col| {
//         (0..=size).for_each(|row| {
//             match pointmap.get(&Point2D(row, col)) {
//                 Some(v) => print!("{}", if *v > 9 { 9 } else { *v }),
//                 None => print!("."),
//             };
//         });
//         println!();
//     });
// }

fn part2(vectors: &Vec<(Point2D, Point2D)>) {
    let points: Vec<Point2D> = vectors
        .iter()
        .map(|(p1, p2)| resolve_vector_points(p1, p2))
        .flatten()
        .collect();
    let mut pointmap: HashMap<Point2D, i64> = HashMap::new();
    points.into_iter().for_each(|p| {
        pointmap.entry(p).and_modify(|v| *v += 1).or_insert(1);
    });

    // printfield(9, &pointmap);

    let sum: i64 = pointmap
        .iter()
        .map(|(_, c)| c)
        .filter(|c| **c > 1)
        .map(|_| 1)
        .sum();
    println!("## Part 2");
    println!("Sum: {}", sum);
}

fn resolve_vector_points(p1: &Point2D, p2: &Point2D) -> Vec<Point2D> {
    if p1.0 == p2.0 {
        get_range(p1.1, p2.1)
            .map(|y| Point2D(p1.0, y))
            .collect::<Vec<Point2D>>()
    } else if p1.1 == p2.1 {
        get_range(p1.0, p2.0)
            .map(|x| Point2D(x, p1.1))
            .collect::<Vec<Point2D>>()
    } else {
        get_range(p1.0, p2.0)
            .zip(get_range(p1.1, p2.1))
            .map(|(x, y)| Point2D(x, y))
            .collect::<Vec<Point2D>>()
    }
}
