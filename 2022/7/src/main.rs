use std::{env, fs, path::Path};

#[derive(Debug)]
enum NodeType {
    File,
    Folder,
}

#[derive(Debug)]
struct Node {
    _name: String,
    nodes: Vec<Node>,
    node_type: NodeType,
    size: u128,
}

impl Node {
    fn sum(&self, max_threshold: u128, total: &mut u128) -> u128 {
        match self.node_type {
            NodeType::File => self.size,
            NodeType::Folder => {
                let sum = self
                    .nodes
                    .iter()
                    .fold(0, |sum, n| sum + n.sum(max_threshold, total));
                if sum <= max_threshold {
                    *total += sum;
                }
                sum
            }
        }
    }

    pub(crate) fn collect_sums(&self, sums: &mut Vec<u128>) -> u128 {
        match self.node_type {
            NodeType::File => self.size,
            NodeType::Folder => {
                let sum = self
                    .nodes
                    .iter()
                    .fold(0, |sum, n| sum + n.collect_sums(sums));
                sums.push(sum);
                sum
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data: String = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<&str> = data.split('\n').filter(|l| !l.is_empty()).skip(1).collect();

    let mut tree = Node {
        _name: "root".to_owned(),
        nodes: vec![],
        node_type: NodeType::Folder,
        size: 0,
    };
    collect_nodes(&mut data.iter(), &mut tree);

    let mut filtered_total = 0;
    let total = tree.sum(100000, &mut filtered_total);
    println!("## Part 1");
    println!("{}, {}", total, filtered_total);

    let left = 70000000 - total;
    let needed = 30000000 - left;

    let mut sums: Vec<u128> = vec![];
    tree.collect_sums(&mut sums);
    let mut sums: Vec<u128> = sums.into_iter().filter(|s| *s >= needed).collect();
    sums.sort();

    println!("## Part 2");
    println!("{}", sums[0]);
}

fn collect_nodes(lines: &mut std::slice::Iter<&str>, tree: &mut Node) {
    for _ in 0..lines.len() {
        let l = lines.next();
        match l {
            Some(cmd) => match *cmd {
                "$ ls" => (),
                "$ cd .." => break,
                _ if cmd.starts_with("$ cd") => {
                    let name = cmd[5..].to_owned();
                    let mut subtree = Node {
                        _name: name.clone(),
                        nodes: vec![],
                        node_type: NodeType::Folder,
                        size: 0,
                    };
                    collect_nodes(lines, &mut subtree);
                    tree.nodes.push(subtree);
                }
                _ if cmd.starts_with("dir ") => (),
                f => {
                    let mut s = f.split(' ');
                    let size = s.next().unwrap();
                    let size: u128 = size.parse().unwrap();
                    tree.nodes.push(Node {
                        _name: s.next().unwrap().to_owned(),
                        nodes: vec![],
                        node_type: NodeType::File,
                        size,
                    })
                }
            },
            _ => break,
        };
    }
}
