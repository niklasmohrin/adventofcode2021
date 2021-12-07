use std::cmp;
use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let stdin = io::stdin();
    let mut positions = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(usize::from_str)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    // Part 1
    positions.sort_unstable();
    let best_position = positions[positions.len() / 2];
    let best_cost: usize = positions
        .iter()
        .map(|&p| cmp::max(p, best_position) - cmp::min(p, best_position))
        .sum();
    dbg!(best_position, best_cost);

    // Part 2
    let last_position = *positions.last().unwrap();
    let costs_for_dist: Vec<usize> = (0..=last_position)
        .scan(0, |acc, cur| {
            *acc += cur;
            Some(*acc)
        })
        .collect();
    let cost = |target_pos| -> usize {
        positions
            .iter()
            .map(|&p| costs_for_dist[cmp::max(target_pos, p) - cmp::min(target_pos, p)])
            .sum()
    };
    let (best_position, best_cost) = (0..=last_position)
        .map(|p| (p, cost(p)))
        .min_by_key(|(_, c)| *c)
        .unwrap();
    dbg!(best_position, best_cost);
}
