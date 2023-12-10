use std::{collections::HashSet, env, fs, path::Path};

#[derive(Debug, Eq, Hash, PartialEq)]
enum Connections {
    L,
    R,
    U,
    D,
}
#[derive(Debug)]
struct Connector {
    symbol: char,
    connections: [Connections; 2],
}

const CONNECTORS: [Connector; 6] = [
    Connector {
        symbol: '|',
        connections: [Connections::U, Connections::D],
    },
    Connector {
        symbol: '-',
        connections: [Connections::L, Connections::R],
    },
    Connector {
        symbol: 'L',
        connections: [Connections::U, Connections::R],
    },
    Connector {
        symbol: 'J',
        connections: [Connections::U, Connections::L],
    },
    Connector {
        symbol: '7',
        connections: [Connections::L, Connections::D],
    },
    Connector {
        symbol: 'F',
        connections: [Connections::R, Connections::D],
    },
];

struct Map {
    tiles: Vec<String>,
    visited: Vec<(usize, usize)>,
}

impl Map {
    fn start(&self) -> Option<(usize, usize)> {
        self.tiles.iter().enumerate().find_map(|(y, l)| {
            l.chars()
                .enumerate()
                .find_map(|(x, c)| if c == 'S' { Some((x, y)) } else { None })
        })
    }

    fn symbol(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || x >= self.tiles[0].len() as i32 || y < 0 || y >= self.tiles.len() as i32 {
            None
        } else {
            self.tiles[y as usize].chars().nth(x as usize)
        }
    }

    fn connector(&self, symbol: char) -> Option<&Connector> {
        CONNECTORS.iter().find(|c| c.symbol == symbol)
    }

    fn resolve_start_symbol(&self, neighbours: &[(&Connector, i32, i32)]) -> Option<char> {
        for n1 in neighbours {
            for n2 in neighbours {
                if n1.1 == n2.1 && n1.2 == n2.2 {
                    continue;
                }
                if let Some(symbol) = match (n1, n2) {
                    ((c1, 1, 0), _) if c1.connections.contains(&Connections::L) => match n2 {
                        (c2, -1, 0) if c2.connections.contains(&Connections::R) => Some('-'),
                        (c2, 0, -1) if c2.connections.contains(&Connections::U) => Some('F'),
                        (c2, 0, 1) if c2.connections.contains(&Connections::D) => Some('L'),
                        _ => None,
                    },
                    ((c1, -1, 0), _) if c1.connections.contains(&Connections::R) => match n2 {
                        (c2, 1, 0) if c2.connections.contains(&Connections::L) => Some('-'),
                        (c2, 0, -1) if c2.connections.contains(&Connections::D) => Some('J'),
                        (c2, 0, 1) if c2.connections.contains(&Connections::U) => Some('7'),
                        _ => None,
                    },
                    ((c1, 0, 1), _) if c1.connections.contains(&Connections::U) => match n2 {
                        (c2, -1, 0) if c2.connections.contains(&Connections::R) => Some('J'),
                        (c2, 0, -1) if c2.connections.contains(&Connections::D) => Some('|'),
                        (c2, 1, 0) if c2.connections.contains(&Connections::L) => Some('F'),
                        _ => None,
                    },
                    ((c1, 0, -1), _) if c1.connections.contains(&Connections::D) => match n2 {
                        (c2, -1, 0) if c2.connections.contains(&Connections::R) => Some('7'),
                        (c2, 1, 0) if c2.connections.contains(&Connections::L) => Some('F'),
                        (c2, 0, 1) if c2.connections.contains(&Connections::U) => Some('|'),
                        _ => None,
                    },
                    _ => None,
                } {
                    return Some(symbol);
                }
            }
        }
        None
    }

    fn ways(&mut self, (sx, sy): (usize, usize)) -> Vec<(usize, usize)> {
        let Some(symbol) = self.symbol(sx as i32, sy as i32) else {
            panic!("no symbol at {},{}", sx, sy);
        };
        let dirs: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let neighbours: Vec<(&Connector, i32, i32)> = dirs
            .iter()
            .flat_map(|(dx, dy)| {
                let nx = sx as i32 + dx;
                let ny = sy as i32 + dy;
                self.symbol(nx, ny)
                    .and_then(|s| self.connector(s).map(|c| (c, *dx, *dy)))
            })
            .collect();

        let current = match symbol {
            'S' => self.resolve_start_symbol(&neighbours),
            s => Some(s),
        }
        .unwrap();

        let result = neighbours
            .iter()
            .flat_map(|(c, dx, dy)| {
                let x = sx as i32 + dx;
                let y = sy as i32 + dy;
                if self.visited.contains(&(x as usize, y as usize)) {
                    return None;
                }
                match (current, dx, dy) {
                    ('|', 0, -1) | ('L', 0, -1) | ('J', 0, -1)
                        if c.connections.contains(&Connections::D) =>
                    {
                        Some((x as usize, y as usize))
                    }
                    ('|', 0, 1) | ('7', 0, 1) | ('F', 0, 1)
                        if c.connections.contains(&Connections::U) =>
                    {
                        Some((x as usize, y as usize))
                    }
                    ('-', -1, 0) | ('J', -1, 0) | ('7', -1, 0)
                        if c.connections.contains(&Connections::R) =>
                    {
                        Some((x as usize, y as usize))
                    }
                    ('-', 1, 0) | ('L', 1, 0) | ('F', 1, 0)
                        if c.connections.contains(&Connections::L) =>
                    {
                        Some((x as usize, y as usize))
                    }
                    _ => None,
                }
            })
            .collect();

        for &(x, y) in &result {
            self.visited.push((x, y));
        }
        result
    }

    fn inside_tiles(&mut self) -> usize {
        let vset: HashSet<&(usize, usize)> = self.visited.iter().collect();

        let h = self.tiles.len();
        let w = self.tiles[0].len();
        let mut count = 0;
        for y in 0..h {
            let mut inside = false;
            dbg!(&self.tiles[y]);
            for x in 0..w {
                let symbol = self.symbol(x as i32, y as i32).unwrap();
                if symbol == 'S' || vset.contains(&(x, y)) {
                    self.tiles[y].replace_range(x..=x, "#");
                    match symbol {
                        'S' | '|' | 'F' | '7' => inside = !inside,
                        _ => (),
                    }
                    dbg!(inside, symbol);
                } else {
                    match symbol {
                        _ if inside => {
                            self.tiles[y].replace_range(x..=x, "O");
                            count += 1;
                        }
                        _ => (),
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let mut data: Map = Map {
        tiles: data
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| l.to_owned())
            .collect(),
        visited: vec![],
    };

    println!("## Part 1");
    part1(&mut data);

    println!("## Part 2");
    part2(&mut data);
}

fn part1(data: &mut Map) {
    let start = data.start().unwrap();
    let mut steps = 0;

    let mut ways = data.ways(start);
    loop {
        steps += 1;
        ways = ways.iter().flat_map(|w| data.ways(*w)).collect();
        if ways.is_empty() {
            break;
        }
    }
    println!("{:?}", steps);
}

fn part2(data: &mut Map) {
    println!("{}", data.inside_tiles());
    dbg!(&data.tiles);
}
