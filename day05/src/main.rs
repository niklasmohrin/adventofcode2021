use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use derive_more::{AddAssign, Sub};
use serde_derive::Deserialize;
use serde_scan::scan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, AddAssign, Sub)]
struct Vec2 {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, Deserialize)]
struct Line(Vec2, Vec2);

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|line| {
        let input = &line.unwrap();
        scan!("{},{} -> {},{}" <- input).unwrap()
    });

    let mut counts = HashMap::<Vec2, usize>::new();
    for Line(start, end) in lines {
        let dir = end - start;
        let dir = Vec2 {
            x: dir.x.signum(),
            y: dir.y.signum(),
        };

        // =======
        // Part 1
        // if dir.x != 0 && dir.y != 0 {
        //     continue;
        // }
        // =======

        let mut cur = start;
        *counts.entry(cur).or_default() += 1;
        while cur != end {
            cur += dir;
            *counts.entry(cur).or_default() += 1;
        }
    }

    println!("{}", counts.values().filter(|&&c| c > 1).count());
}
