use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use counter::Counter;

fn main() {
    let stdin = io::stdin();
    let input: Vec<String> = stdin.lock().lines().map(Result::unwrap).collect();

    let rules: HashMap<(char, char), char> = input[2..]
        .iter()
        .map(|line| {
            let (l, r) = line.split_once(" -> ").unwrap();
            (
                (l.chars().nth(0).unwrap(), l.chars().nth(1).unwrap()),
                r.chars().next().unwrap(),
            )
        })
        .collect();

    fn visit<'a>(
        node: (char, char),
        depth: usize,
        rules: &HashMap<(char, char), char>,
        cache: &'a mut HashMap<((char, char), usize), Counter<char>>,
    ) -> &'a Counter<char> {
        if !cache.contains_key(&(node, depth)) {
            let mut c = Counter::new();
            if depth != 0 {
                if let Some(&inserted) = rules.get(&node) {
                    c[&inserted] += 1;
                    c.extend(visit((node.0, inserted), depth - 1, rules, cache));
                    c.extend(visit((inserted, node.1), depth - 1, rules, cache));
                }
            }

            cache.insert((node, depth), c);
        }

        cache.get(&(node, depth)).unwrap()
    }

    let mut counts: Counter<_> = input[0].chars().collect();
    let initial_chars: Vec<_> = input[0].chars().collect();
    for win in initial_chars.windows(2) {
        counts.extend(visit((win[0], win[1]), 40, &rules, &mut HashMap::new()));
    }

    let counts = counts.most_common();
    dbg!(counts.first().unwrap().1 - counts.last().unwrap().1);
}
