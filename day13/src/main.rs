use std::{
    cmp,
    collections::HashSet,
    io::{self, Read},
};

use serde_scan::scan;

fn main() {
    let stdin = io::stdin();
    let input = {
        let mut s = String::new();
        stdin.lock().read_to_string(&mut s).unwrap();
        s
    };

    let (points, folds) = input.split_once("\n\n").unwrap();

    let points = points.lines().map(|l| scan!("{},{}" <- l).unwrap());
    let folds: Vec<(char, usize)> = folds
        .lines()
        .map(|l| scan!("fold along {}={}" <- l).unwrap())
        .collect();

    let fold_one = |v, axis| cmp::min(v, 2 * axis - v);
    let apply_all_folds = |initial_point| {
        folds
            .iter()
            // .take(1) // <- For part 1, uncomment
            .fold(initial_point, |(x, y), &(dim, axis)| {
                if dim == 'x' {
                    (fold_one(x, axis), y)
                } else {
                    (x, fold_one(y, axis))
                }
            })
    };

    let final_points: HashSet<_> = points.map(apply_all_folds).collect();
    dbg!(final_points.len());

    // Plot the points
    let &width = final_points.iter().map(|(x, _)| x).max().unwrap();
    let &height = final_points.iter().map(|(_, y)| y).max().unwrap();

    for y in 0..=height {
        for x in 0..=width {
            if final_points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
