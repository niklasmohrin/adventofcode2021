#![feature(array_windows)]

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let depths = stdin
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect::<Vec<usize>>();

    // Part 1
    // let increasing = depths.array_windows().filter(|[a, b]| a < b).count();
    // Part 2
    let increasing = depths
        .array_windows::<4>()
        .filter(|[a, _, _, b]| a < b)
        .count();
    println!("{}", increasing);
}
