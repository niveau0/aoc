use std::{cmp::Ordering, collections::HashSet, env, fs, path::Path};

struct Snake {
    parts: Vec<(i32, i32)>,
    tail_visits: HashSet<(i32, i32)>,
}

impl Snake {
    fn new(len: usize) -> Self {
        if len < 2 {
            panic!("Dead snake, no tail");
        }
        let parts = (0..len).map(|_| (60, 60)).collect();
        Snake {
            parts,
            tail_visits: HashSet::new(),
        }
    }

    fn move_tail(&mut self) {
        for i in 1..self.parts.len() {
            let prev = self.parts[i - 1];
            let mut part = self.parts[i];
            if part.0 + 1 < prev.0 {
                part.0 += 1;
                follow(&mut part.1, prev.1);
            }
            if part.0 - 1 > prev.0 {
                part.0 -= 1;
                follow(&mut part.1, prev.1);
            }
            if part.1 + 1 < prev.1 {
                part.1 += 1;
                follow(&mut part.0, prev.0);
            }
            if part.1 - 1 > prev.1 {
                part.1 -= 1;
                follow(&mut part.0, prev.0);
            }
            self.parts[i] = part;
        }
        self.tail_visits.insert(*self.parts.last().unwrap());
    }

    fn right(&mut self) {
        let mut h = self.parts[0];
        h.0 += 1;
        self.parts[0] = h;
        self.move_tail();
    }
    fn down(&mut self) {
        let mut h = self.parts[0];
        h.1 += 1;
        self.parts[0] = h;
        self.move_tail();
    }
    fn left(&mut self) {
        let mut h = self.parts[0];
        h.0 -= 1;
        self.parts[0] = h;
        self.move_tail();
    }
    fn up(&mut self) {
        let mut h = self.parts[0];
        h.1 -= 1;
        self.parts[0] = h;
        self.move_tail();
    }

    fn print(&self) {
        for r in 0..120 {
            for c in 0..120 {
                if self.parts.contains(&(c, r)) {
                    let num = self
                        .parts
                        .iter()
                        .enumerate()
                        .find_map(|(idx, p)| {
                            if p.0 == c && p.1 == r {
                                Some(idx)
                            } else {
                                None
                            }
                        })
                        .unwrap();
                    print!("{}", num)
                } else {
                    print!(".")
                }
            }
            println!();
        }
        println!();
    }

    fn print_visits(&self) {
        for r in 0..120 {
            for c in 0..120 {
                if self.tail_visits.contains(&(c, r)) {
                    print!("x")
                } else {
                    print!(".")
                }
            }
            println!();
        }
        println!();
    }
}

fn follow(part: &mut i32, prev: i32) {
    match (*part).cmp(&prev) {
        Ordering::Less => *part += 1,
        Ordering::Greater => *part -= 1,
        Ordering::Equal => (),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<(char, i32)> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .flat_map(|s| s.split_once(' '))
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();

    let mut simu = Snake::new(2);
    run(&data, &mut simu);

    println!("## Part 1");
    println!("visits: {}", simu.tail_visits.len());

    let mut simu = Snake::new(10);
    run(&data, &mut simu);
    simu.print();

    println!("## Part 2");
    println!("visits: {}", simu.tail_visits.len());
}

fn run(data: &[(char, i32)], simu: &mut Snake) {
    data.iter().for_each(|cmd| {
        //println!("{} {}", &cmd.0, &cmd.1);
        match cmd {
            ('R', m) => (0..*m).for_each(|_| simu.right()),
            ('L', m) => (0..*m).for_each(|_| simu.left()),
            ('U', m) => (0..*m).for_each(|_| simu.up()),
            ('D', m) => (0..*m).for_each(|_| simu.down()),
            _ => panic!("Illegal command"),
        }
    });
    simu.print_visits();
}
