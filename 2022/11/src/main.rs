use std::{env, fs, path::Path};

#[derive(Clone, Debug)]
enum Operation {
    MulSelf,
    Mul(u64),
    Add(u64),
}
impl Operation {
    fn apply(&self, item: u64) -> u64 {
        match self {
            Operation::MulSelf => item * item,
            Operation::Mul(v) => item * v,
            Operation::Add(v) => item + v,
        }
    }
}

#[derive(Clone, Debug)]
struct Rule {
    modulo: u64,
    targets: (usize, usize),
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    rule: Rule,
    count: usize,
}

impl Monkey {
    fn new(items: Vec<u64>, op: Operation, rule: Rule) -> Self {
        Monkey {
            items,
            op,
            rule,
            count: 0,
        }
    }

    fn play(&self, my_idx: usize, data: &mut Vec<Monkey>, divide: u64) {
        let modulo: u64 = data.iter().map(|m| m.rule.modulo).product();
        self.items.iter().for_each(|item| {
            let mut worried = self.op.apply(*item) / divide;
            // Stuṕid solution: requies mathmatical instead of programming skills, real time waste
            worried %= modulo; // part 2
            if worried % self.rule.modulo == 0 {
                data[self.rule.targets.0].add(worried);
            } else {
                data[self.rule.targets.1].add(worried);
            }
        });
        data[my_idx].count += self.items.len() as usize;
        data[my_idx].items = vec![];
    }

    fn add(&mut self, item: u64) {
        self.items.push(item)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<Monkey> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .chunks(6)
        .map(|m| {
            Monkey::new(
                m[1]["  Starting items: ".len()..]
                    .split(", ")
                    .map(|i| i.parse().unwrap())
                    .collect(),
                match m[2]["  Operation: new = old ".len()..].split_once(' ') {
                    Some(("+", v)) => Operation::Add(v.parse().unwrap()),
                    Some(("*", "old")) => Operation::MulSelf,
                    Some(("*", v)) => Operation::Mul(v.parse().unwrap()),
                    o => panic!("illegal operation {:?}", o),
                },
                Rule {
                    modulo: m[3]["  Test: divisible by ".len()..].parse().unwrap(),
                    targets: (
                        m[4]["    If true: throw to monkey ".len()..]
                            .parse()
                            .unwrap(),
                        m[5]["    If false: throw to monkey ".len()..]
                            .parse()
                            .unwrap(),
                    ),
                },
            )
        })
        .collect();

    let mut part1 = data.clone();
    let mut part2 = data.clone();

    run(&mut part1, 20, 3);

    part1.sort_by(|a, b| (*a).count.partial_cmp(&(*b).count).unwrap());
    part1.reverse();
    println!("## Part 1");
    println!("total: {}", part1[0].count * part1[1].count);

    run(&mut part2, 10000, 1);

    part2.sort_by(|a, b| (*a).count.partial_cmp(&(*b).count).unwrap());
    part2.reverse();

    println!("## Part 2");
    println!("total: {}", part2[0].count * part2[1].count);
}

fn run(data: &mut Vec<Monkey>, turns: i32, divide: u64) {
    (0..turns).for_each(|_| {
        (0..data.len()).for_each(|idx| data[idx].clone().play(idx, data, divide));
    });
}
