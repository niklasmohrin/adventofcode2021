use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();

    let opening_characters = HashSet::from(['(', '[', '{', '<']);
    let opening_for =
        HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')].map(|(l, r)| (r, l)));
    let unexpected_error_score = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let missing_error_score = HashMap::from([('(', 1u64), ('[', 2), ('{', 3), ('<', 4)]);

    let mut total_unexpected_score = 0;
    let mut all_missing_scores = Vec::new();

    'outer: for line in stdin.lock().lines() {
        let mut stack = Vec::new();
        for c in line.unwrap().chars() {
            if opening_characters.contains(&c) {
                stack.push(c);
            } else if let Some(last) = stack.pop() {
                if last != opening_for[&c] {
                    total_unexpected_score += unexpected_error_score[&c];
                    continue 'outer;
                }
            }
        }

        all_missing_scores.push(
            stack
                .into_iter()
                .rev()
                .map(|b| missing_error_score[&b])
                .fold(0, |acc, cur| acc * 5 + cur),
        );
    }

    println!("{}", total_unexpected_score);
    all_missing_scores.sort_unstable();
    println!("{}", all_missing_scores[all_missing_scores.len() / 2]);
}
