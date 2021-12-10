use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandType {
    Forward,
    Up,
    Down,
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let commands = stdin
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (cmd, val) = line.split_once(' ').unwrap();
            let val = val.parse().unwrap();
            let cmd = match cmd {
                "forward" => CommandType::Forward,
                "up" => CommandType::Up,
                "down" => CommandType::Down,
                _ => panic!(),
            };
            (cmd, val)
        })
        .collect::<Vec<(CommandType, usize)>>();

    // Part 1
    // let (xs, ys): (Vec<(CommandType, usize)>, Vec<(CommandType, usize)>) = commands
    //     .into_iter()
    //     .partition(|(cmd, _)| *cmd == CommandType::Forward);
    // let x = xs.into_iter().map(|(_, v)| v).sum::<usize>() as isize;
    // let y: isize = ys
    //     .into_iter()
    //     .map(|(cmd, v)| match cmd {
    //         CommandType::Down => v as isize,
    //         CommandType::Up => -(v as isize),
    //         _ => panic!(),
    //     })
    //     .sum();

    // Part 2
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for (cmd, val) in commands {
        match cmd {
            CommandType::Down => aim += val,
            CommandType::Up => aim -= val,
            CommandType::Forward => {
                x += val;
                y += val * aim;
            }
        }
    }

    println!("{}", x * y);
}
