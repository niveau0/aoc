use std::{collections::HashMap, env, fs, path::Path};

#[derive(Debug, Clone)]
struct PathNode(String, Vec<PathNode>);

#[derive(Debug, Clone)]
struct Edge(String, String);

#[derive(Debug, Clone)]
struct Input(Vec<Edge>);

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");

    let edges: Vec<Edge> = data
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .flat_map(|l| {
            let (s, e) = l.split_once("-").unwrap();
            [
                Edge(s.to_owned(), e.to_owned()),
                Edge(e.to_owned(), s.to_owned()),
            ]
            .into_iter()
        })
        .filter(|e| e.1 != "start") // do not allow enter start again
        .filter(|e| e.0 != "end") // do not allow leaving end again
        .collect();

    let input = Input(edges);

    part1(&input);
    part2(&input);
}

fn part1(input: &Input) {
    let mut input = input.clone();

    input.print();

    let pathes = input
        .find_all_path("start", "end", &mut vec![], false)
        .expect("no path?");

    println!("## Part 1");
    println!("Result {}", pathes.count());
}

fn part2(input: &Input) {
    let mut input = input.clone();
    let pathes = input
        .find_all_path("start", "end", &mut vec![], true)
        .expect("no path?");

    println!("## Part 2");
    println!("Result {}", pathes.count());
}

impl Input {
    fn print(&mut self) {
        self.0.iter().for_each(|t| {
            println!("{}-{}", t.0, t.1);
        });
        println!();
    }

    fn find_all_path(
        &mut self,
        from: &str,
        to: &str,
        visited: &mut Vec<String>,
        twice: bool,
    ) -> Option<PathNode> {
        let edges = self.find_edge(from);
        if edges.len() == 0 {
            return None;
        }

        let mut pathes = vec![];
        for edge in edges {
            if edge.1 == to {
                pathes.push(PathNode(to.to_owned(), vec![]));
            } else {
                let smallcave = edge.1.to_lowercase() == *edge.1;
                let subpath = if smallcave {
                    let visits = self.count_visit(&edge.1, visited);
                    if visits < 1 || twice {
                        let twice = twice && visits < 1;
                        let mut visited = visited.clone();
                        visited.push(edge.1.clone());
                        self.find_all_path(&edge.1, to, &mut visited, twice)
                    } else {
                        None
                    }
                } else {
                    self.find_all_path(&edge.1, to, visited, twice)
                };

                if subpath.is_some() {
                    pathes.push(subpath.unwrap());
                }
            }
        }
        if pathes.len() > 0 {
            Some(PathNode(from.to_owned(), pathes))
        } else {
            None
        }
    }

    fn count_visit(&self, cave: &String, visited: &Vec<String>) -> usize {
        visited.iter().filter(|c| **c == *cave).count()
    }

    fn find_edge(&self, from: &str) -> Vec<Edge> {
        self.0.iter().filter(|e| e.0 == from).cloned().collect()
    }
}
impl PathNode {
    fn count(&self) -> u16 {
        self.1.iter().map(|p| p.count()).sum::<u16>() + if self.0 == "end" { 1 } else { 0 }
    }
}
