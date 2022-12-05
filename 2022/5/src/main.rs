use std::{collections::VecDeque, env, fs, path::Path};

type Stack = VecDeque<char>;

#[derive(Debug, Clone)]
struct Ship {
    stacks: Vec<Stack>,
}

#[derive(Debug)]
struct Move {
    times: u8,
    from: usize,
    to: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut ship = Ship { stacks: vec![] };

    let moves: Vec<Move> = data
        .split('\n')
        .skip_while(|l| !l.is_empty())
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(parse_move)
        .collect();

    data.split('\n')
        .take_while(|l| !l.starts_with(" 1 "))
        .for_each(|crates| parse_crates(&mut ship, crates));

    part1(&mut ship.clone(), &moves);

    part2(&mut ship, &moves);
}

fn parse_move(l: &str) -> Move {
    let mut s = l.split(' ');
    s.next();
    let times = s.next().unwrap().parse().unwrap();
    s.next();
    let from = s.next().unwrap().parse().unwrap();
    s.next();
    let to = s.next().unwrap().parse().unwrap();
    Move { times, from, to }
}

fn parse_crates(ship: &mut Ship, crates: &str) {
    let num_stacks = (crates.len() + 1) / 4;
    if ship.stacks.len() < num_stacks {
        ship.stacks = (0..num_stacks).map(|_| VecDeque::new()).collect();
    }
    for i in 0..num_stacks {
        let c = crates.chars().nth(i * 4 + 1);
        if let Some(c) = c {
            if c != ' ' {
                ship.stacks[i].push_back(c)
            }
        }
    }
}

fn part1(ship: &mut Ship, moves: &[Move]) {
    moves.iter().for_each(|m| {
        (0..m.times).for_each(|_| {
            let ship_crate = ship.stacks[m.from - 1].pop_front().unwrap();
            ship.stacks[m.to - 1].push_front(ship_crate);
        })
    });

    println!("## Part 1");
    print_result(ship);
}

fn part2(ship: &mut Ship, moves: &[Move]) {
    moves.iter().for_each(|m| {
        let mut to_move: Vec<char> = (0..m.times)
            .map(|_| ship.stacks[m.from - 1].pop_front().unwrap())
            .collect();
        to_move.reverse();
        to_move.iter().for_each(|c| {
            ship.stacks[m.to - 1].push_front(*c);
        })
    });

    println!("## Part 2");
    print_result(ship);
}

fn print_result(ship: &mut Ship) {
    ship.stacks
        .iter_mut()
        .for_each(|s| print!("{}", s.pop_front().unwrap()));
    println!();
}
