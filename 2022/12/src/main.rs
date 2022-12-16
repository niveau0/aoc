use std::{collections::HashMap, env, fs, path::Path};

fn elevation(letter: &char) -> u8 {
    match *letter {
        'S' => 0,
        'E' => b'z' - b'a',
        l => l as u8 - b'a',
    }
}
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

    let end = find_pos(&data, 'E');
    let start = find_pos(&data, 'S');
    let mut visited = HashMap::new();
    climb(&data, start, end, &mut visited, 0);
    println!("## Part 1");
    println!("total: {}", *visited.get(&end).unwrap_or(&0));

    println!("## Part 2");
    let starts: Vec<(usize, usize)> = data
        .iter()
        .enumerate()
        .flat_map(|(y, r)| {
            let x = r
                .iter()
                .enumerate()
                .filter_map(move |(x, c)| if *c == 'a' { Some((x, y)) } else { None });
            x
        })
        .collect();
    starts
        .iter()
        .for_each(|s| climb(&data, *s, end, &mut visited, 0));
    println!("total: {}", *visited.get(&end).unwrap_or(&0));
}

fn find_pos(data: &[Vec<char>], to_find: char) -> (usize, usize) {
    data.iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find_map(|(x, c)| if *c == to_find { Some(x) } else { None })
                .map(|x| Some((x, y)))
        })
        .unwrap()
        .unwrap()
}

fn climb(
    data: &[Vec<char>],
    pos: (usize, usize),
    end: (usize, usize),
    visited: &mut HashMap<(usize, usize), usize>,
    length: usize,
) {
    let current = elevation(&data[pos.1][pos.0]);
    if pos.0 > 0 && elevation(&data[pos.1][pos.0 - 1]) <= current + 1 {
        step((pos.0 - 1, pos.1), visited, end, data, length)
    };
    if pos.1 > 0 && elevation(&data[pos.1 - 1][pos.0]) <= current + 1 {
        step((pos.0, pos.1 - 1), visited, end, data, length)
    };
    if pos.0 < data[0].len() - 1 && elevation(&data[pos.1][pos.0 + 1]) <= current + 1 {
        step((pos.0 + 1, pos.1), visited, end, data, length)
    };
    if pos.1 < data.len() - 1 && elevation(&data[pos.1 + 1][pos.0]) <= current + 1 {
        step((pos.0, pos.1 + 1), visited, end, data, length)
    };
}

fn step(
    new_pos: (usize, usize),
    visited: &mut HashMap<(usize, usize), usize>,
    end: (usize, usize),
    data: &[Vec<char>],
    length: usize,
) {
    let tracked_length = *visited.get(&new_pos).unwrap_or(&0);
    match new_pos {
        _ if tracked_length != 0 && length + 1 >= tracked_length => (),
        p if p == end => {
            if tracked_length == 0 || tracked_length > length {
                visited.insert(p, length + 1);
                //print(length, data, p);
            }
        }
        p => {
            visited.insert(p, length + 1);
            //print(length, data, p);
            climb(data, p, end, visited, length + 1)
        }
    }
}

// fn print(length: usize, data: &[Vec<char>], pos: (usize, usize)) {
//     let color: &dyn termion::color::Color = match length + 1 {
//         l if l > 35 => &termion::color::Red,
//         l if l > 10 => &termion::color::Cyan,
//         l if l > 5 => &termion::color::Blue,
//         l if l > 1 => &termion::color::Cyan,
//         _ => &termion::color::White,
//     };
//     print!(
//         "{}{}{}",
//         termion::cursor::Goto(pos.0 as u16 + 1, pos.1 as u16 + 1),
//         termion::color::Fg(color),
//         &data[pos.1][pos.0]
//     );
// }
