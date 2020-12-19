use std::{env, fs, path::Path};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let lines = data.split("\n").collect();

    // part1(&lines);
    part1_with_waypoint(&lines);
    part2(&lines);
}

// fn do_move1(cmd: &str, range: i32, pos: &mut (i32, i32), wpos: &(i32, i32)) {
//     match cmd {
//         "N" => pos.1 = pos.1 + range * wpos.1,
//         "E" => pos.0 = pos.0 + range * wpos.0,
//         "S" => pos.1 = pos.1 - range * wpos.1,
//         "W" => pos.0 = pos.0 - range * wpos.0,
//         _ => panic!("Bad command {}", cmd),
//     }
// }

fn do_move(range: i32, pos: &mut (i32, i32), wpos: &(i32, i32)) {
    pos.0 = pos.0 + range * wpos.0;
    pos.1 = pos.1 + range * wpos.1;
    // println!("p {:?} w {:?}", &pos, &wpos);
}

// fn part1(lines: &Vec<&str>) {
//     let mut pos = (0, 0);
//     let mut wpos = (1, 1);

//     let mut face = "E";
//     let regex = Regex::new(r"([^0-9]+)(\d+)").unwrap();
//     regex.captures("");
//     for dir in lines {
//         let matches = regex.captures(dir.trim()).unwrap();
//         let cmd = matches.get(1).unwrap().as_str();
//         let range: i32 = matches.get(2).unwrap().as_str().parse().unwrap();
//         match cmd {
//             "F" => {
//                 do_move1(face, range, &mut pos, &wpos);
//             }
//             "R" => (0..(range / 90)).for_each(|_| {
//                 face = match face {
//                     "N" => "E",
//                     "E" => "S",
//                     "S" => "W",
//                     "W" => "N",
//                     _ => panic!("Bad face"),
//                 }
//             }),
//             "L" => (0..(range / 90)).for_each(|_| {
//                 face = match face {
//                     "E" => "N",
//                     "S" => "E",
//                     "W" => "S",
//                     "N" => "W",
//                     _ => panic!("Bad face"),
//                 }
//             }),
//             _ => do_move1(cmd, range, &mut pos, &wpos),
//         }
//     }
//     println!("## Part 1");

//     println!(
//         "Abs(Pos {} + {}) = {}",
//         pos.0,
//         pos.1,
//         pos.0.abs() + pos.1.abs()
//     )
// }

fn part1_with_waypoint(lines: &Vec<&str>) {
    let mut pos = (0, 0);
    let mut wpos = (1, 0);
    let regex = Regex::new(r"([^0-9]+)(\d+)").unwrap();
    regex.captures("");
    for dir in lines {
        let matches = regex.captures(dir.trim()).unwrap();
        let cmd = matches.get(1).unwrap().as_str();
        let range: i32 = matches.get(2).unwrap().as_str().parse().unwrap();
        match cmd {
            "F" => do_move(range, &mut pos, &wpos),
            "R" => (0..(range / 90)).for_each(|_| wpos = (wpos.1, -wpos.0)),
            "L" => (0..(range / 90)).for_each(|_| wpos = (-wpos.1, wpos.0)),
            "N" => do_move(range, &mut pos, &(0, 1)),
            "E" => do_move(range, &mut pos, &(1, 0)),
            "S" => do_move(range, &mut pos, &(0, -1)),
            "W" => do_move(range, &mut pos, &(-1, 0)),
            _ => panic!("Bad command {}", cmd),
        }
    }

    println!("## Part 1");

    println!(
        "Abs(Pos {} + {}) = {}",
        pos.0,
        pos.1,
        pos.0.abs() + pos.1.abs()
    )
}

fn part2(lines: &Vec<&str>) {
    let mut pos = (0, 0);
    let mut wpos = (10, 1);
    let regex = Regex::new(r"([^0-9]+)(\d+)").unwrap();
    regex.captures("");
    for dir in lines {
        let matches = regex.captures(dir.trim()).unwrap();
        let cmd = matches.get(1).unwrap().as_str();
        let range: i32 = matches.get(2).unwrap().as_str().parse().unwrap();
        match cmd {
            "F" => do_move(range, &mut pos, &wpos),
            "R" => (0..(range / 90)).for_each(|_| wpos = (wpos.1, -wpos.0)),
            "L" => (0..(range / 90)).for_each(|_| wpos = (-wpos.1, wpos.0)),
            "N" => do_move(range, &mut wpos, &(0, 1)),
            "E" => do_move(range, &mut wpos, &(1, 0)),
            "S" => do_move(range, &mut wpos, &(0, -1)),
            "W" => do_move(range, &mut wpos, &(-1, 0)),
            _ => panic!("Bad command {}", cmd),
        }
    }

    println!("## Part 2");

    println!(
        "Abs(Pos {} + {}) = {}",
        pos.0,
        pos.1,
        pos.0.abs() + pos.1.abs()
    )
}
