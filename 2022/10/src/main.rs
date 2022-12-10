use std::{env, fs, path::Path};

#[derive(Clone, Debug)]
enum Command {
    Noop,
    AddX(i64),
}

struct Cpu {
    cmd: Option<Command>,
    x: i64,
    exec_time: u8,
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            cmd: None,
            x: 1,
            exec_time: 0,
        }
    }

    fn load_command(&mut self, command: &Command) {
        self.cmd = Some(command.clone());
        self.exec_time = match command {
            Command::Noop => 1,
            Command::AddX(_) => 2,
        }
    }

    fn tick(&mut self) {
        if self.exec_time > 1 {
            self.exec_time -= 1;
            return;
        }
        if let Some(cmd) = &self.cmd {
            match cmd {
                Command::Noop => (),
                Command::AddX(v) => self.x += v,
            }
            self.cmd = None;
        }
    }

    fn is_idle(&self) -> bool {
        self.cmd.is_none()
    }

    fn x_register(&self) -> i64 {
        self.x
    }
}

struct Crt {
    screen: Vec<char>,
}

impl Crt {
    fn new() -> Self {
        Crt {
            screen: vec!['.'; 240],
        }
    }

    fn mark(&mut self, pos: usize) {
        if pos >= self.screen.len() {
            panic!("out of screen");
        }
        self.screen[pos] = 'x';
    }

    fn print(&self) {
        self.screen.iter().enumerate().for_each(|(pos, pixel)| {
            if pos % 40 == 0 {
                println!();
            }
            print!("{}", pixel);
        });
        println!();
    }
}

fn run(mut cmds: std::slice::Iter<Command>, cycles: u32) -> (i64, Crt) {
    let mut cpu = Cpu::new();
    let mut total = 0;
    let mut crt = Crt::new();

    (1..=cycles).map(|c| c as i64).for_each(|cycle| {
        if cpu.is_idle() {
            cpu.load_command(cmds.next().unwrap());
        }
        if (cycle - 20) % 40 == 0 {
            total += cycle * cpu.x_register();
        }
        let crt_pos = cycle - 1;
        if (crt_pos % 40 - cpu.x_register()).abs() < 2 {
            crt.mark(crt_pos as usize);
        }
        cpu.tick();
    });
    (total, crt)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file = args.get(1).expect("Missing file parameter");
    if !Path::new(file).exists() {
        panic!("No such file {}", file);
    }
    let data = fs::read_to_string(file).expect("Something went wrong reading the file");
    let data: Vec<Command> = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|s| match s {
            "noop" => Command::Noop,
            _ if s.starts_with("addx ") => Command::AddX(s[5..].parse().unwrap()),
            _ => panic!("illegal command"),
        })
        .collect();

    let cmds = data.iter();
    let (total, _) = run(cmds, 220);
    println!("## Part 1");
    println!("total: {}", total);

    let cmds = data.iter();
    let (_, crt) = run(cmds, 240);
    println!("## Part 2");
    crt.print();
}
