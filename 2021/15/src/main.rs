use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    env, fs,
    path::Path,
    rc::Rc,
};

const NEIGHBOURS: [(i16, i16); 4] = [(0, -1), (-1, 0), (0, 1), (1, 0)];

#[derive(Debug, Clone)]
struct Input(Vec<Vec<u16>>);

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");

    let input = Input(
        data.split("\n")
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| {
                l.split("")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(|s| s.parse::<u16>().unwrap())
                    .collect::<Vec<u16>>()
            })
            .collect(),
    );

    part1(&input);
    part2(&input);
}

#[derive(Debug)]
struct Grid(Vec<Vec<Node>>);

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash)]
struct Node {
    x: usize,
    y: usize,
    g: u16,
    h: u16,
}

#[derive(Debug, PartialEq, Eq)]
struct PathNode<'a>(&'a Node, u16, Option<Rc<PathNode<'a>>>);

impl<'a> PartialOrd for PathNode<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let PathNode(selfnode, selfcost, _) = self;
        let PathNode(othernode, othercost, _) = other;

        Some(if othernode.h + othercost == selfnode.h + selfcost {
            Ordering::Equal
        } else if othernode.h + othercost < selfnode.h + selfcost {
            Ordering::Less
        } else {
            Ordering::Greater
        })
    }
}

impl<'a> Ord for PathNode<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let PathNode(selfnode, selfcost, _) = self;
        let PathNode(othernode, othercost, _) = other;

        dbg!("ord");
        if othernode.h + othercost == selfnode.h + selfcost {
            Ordering::Equal
        } else if othernode.h + othercost < selfnode.h + selfcost {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

fn part1(input: &Input) {
    let grid = Grid::new(&input);
    let (w, h) = grid.size();
    let goal = grid.node(w - 1, h - 1).unwrap();
    let start = PathNode(&grid.node(0, 0).unwrap(), 0, None);

    let last = find_path(start, goal, &grid);
    let (path, risk) = aggregate(last, &grid);

    println!("## Part 1");
    if path.len() == 0 {
        println!("No path found");
    } else {
        println!("Risk: {}", risk);
    }
}

fn part2(input: &Input) {
    let input = add_tiles(&input);
    let grid = Grid::new(&input);
    let (w, h) = grid.size();
    let goal = grid.node(w - 1, h - 1).unwrap();
    let start = PathNode(&grid.node(0, 0).unwrap(), 0, None);

    let last = find_path(start, goal, &grid);
    let (path, risk) = aggregate(last, &grid);

    println!("## Part 2");
    if path.len() == 0 {
        println!("No path found");
    } else {
        println!("Risk: {}", risk);
    }
}

fn aggregate<'a>(last: Option<PathNode<'a>>, grid: &'a Grid) -> (Vec<&'a Node>, u16) {
    let mut path = vec![];
    let mut risk = 0;
    if let Some(PathNode(n, _, parent)) = last {
        risk += n.g;
        path.push(n);

        let mut parent = &parent;
        loop {
            if let Some(pathnode) = &parent {
                let PathNode(n, _, p) = &**pathnode;
                path.push(n);
                parent = p;
                if n.x != 0 || n.y != 0 {
                    risk += n.g;
                }
            } else {
                break;
            }
        }
        grid.print(&path);
    }
    (path, risk)
}

fn add_tiles(input: &Input) -> Input {
    let tiles = 5;
    let (w, h) = input.size();

    let mut field: Vec<Vec<u16>> = Vec::with_capacity(h * tiles);
    (0..h * tiles).for_each(|_| field.push(vec![0; w * tiles]));

    (0..tiles).for_each(|tile_y| {
        (0..h).for_each(|y| {
            (0..tiles).for_each(|tile_x| {
                (0..w).for_each(|x| {
                    let mut v = (input.0[y][x] + tile_y as u16 + tile_x as u16) % 9;
                    if v == 0 {
                        v = 9;
                    }
                    field[y + h * tile_y][x + w * tile_x] = v;
                })
            })
        })
    });

    Input(field)
}

fn find_path<'a>(start: PathNode<'a>, goal: &Node, grid: &'a Grid) -> Option<PathNode<'a>> {
    let mut open = BinaryHeap::new();
    open.push(start);
    let mut visited: HashMap<(usize, usize), Rc<PathNode>> = HashMap::new();
    let mut last = None;
    loop {
        let next = open.pop();
        match next {
            None => {
                break;
            }
            Some(PathNode(&Node { x, y, g: _, h: _ }, _cost, _)) if x == goal.x && y == goal.y => {
                last = next;
                break;
            }
            Some(p) => {
                let PathNode(Node { x, y, g: _, h: _ }, cost, _) = p;
                let neighbours = grid.neighbours(*x, *y);
                let p = Rc::new(p);
                neighbours
                    .into_iter()
                    .filter(|n| visited.get(&(n.x, n.y)).is_none())
                    .for_each(|n| {
                        let nextcost = cost + n.g;
                        open.push(PathNode(n, nextcost, Some(p.clone())));
                    });
                visited.insert((*x, *y), p.clone());
            }
        }
    }
    last
}

impl Grid {
    fn new(input: &Input) -> Self {
        let (w, h) = input.size();
        let nodes: Vec<Vec<Node>> = (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| Node {
                        x,
                        y,
                        g: input.0[y][x],
                        h: (((w - x).pow(2) + (h - y).pow(2)) as f64).sqrt() as u16, // A*
                                                                                     //h: 0, // is this Dijkstra?
                    })
                    .collect::<Vec<Node>>()
            })
            .collect();

        Grid(nodes)
    }

    fn print(&self, path: &Vec<&Node>) {
        let (w, h) = self.size();

        let pathmap: HashMap<&Node, bool> = path.iter().map(|n| (*n, true)).collect();
        (0..h).for_each(|y| {
            (0..w).for_each(|x| {
                let node = self.node(x, y).unwrap();

                let mark = pathmap.get(node).is_some();
                if mark {
                    print!("\x1b[1m{}", node.g);
                } else {
                    print!("\x1b[0m{}", node.g);
                }
            });
            println!();
        });
        println!();
    }

    fn size(&self) -> (usize, usize) {
        (self.0[0].len(), self.0.len())
    }

    fn neighbours<'a>(&self, x: usize, y: usize) -> Vec<&Node> {
        let (w, h) = self.size();

        NEIGHBOURS
            .iter()
            .flat_map(|(nx, ny)| {
                let nx = x as i16 + nx;
                let ny = y as i16 + ny;
                if nx >= 0 && nx < w as i16 && ny >= 0 && ny < h as i16 {
                    Some(&self.0[ny as usize][nx as usize])
                } else {
                    None
                }
            })
            .collect()
    }

    fn node(&self, x: usize, y: usize) -> Option<&Node> {
        let (w, h) = self.size();
        if x >= w || y >= h {
            None
        } else {
            Some(&self.0[y][x])
        }
    }
}

impl Input {
    fn size(&self) -> (usize, usize) {
        (self.0[0].len(), self.0.len())
    }
}
