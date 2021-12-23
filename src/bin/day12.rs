use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap).collect::<Vec<_>>();

    let mut index_of = HashMap::new();
    let mut name_of = Vec::new();
    let mut g = Vec::new();

    for line in lines.iter() {
        let (a, b) = line.split_once('-').unwrap();
        let a = *index_of.entry(a).or_insert_with(|| {
            g.push(Vec::new());
            name_of.push(a);
            g.len() - 1
        });
        let b = *index_of.entry(b).or_insert_with(|| {
            g.push(Vec::new());
            name_of.push(b);
            g.len() - 1
        });
        g[a].push(b);
        g[b].push(a);
    }

    fn can_visit(node: usize, path: &[usize], name_of: &[&str]) -> bool {
        name_of[node].chars().all(|c| c.is_ascii_uppercase()) || !path.contains(&node)
    }
    fn can_visit_small_with_joker(node: usize, path: &[usize], name_of: &[&str]) -> bool {
        !["start", "end"].contains(&name_of[node])
            && path.iter().filter(|&&n| n == node).count() < 2
    }

    fn visit(
        g: &Vec<Vec<usize>>,
        cur: usize,
        path: &mut Vec<usize>,
        name_of: &[&str],
        counts: &mut [usize],
        joker_taken: bool,
    ) {
        counts[cur] += 1;
        path.push(cur);
        for &neigh in g[cur].iter() {
            if can_visit(neigh, path, name_of) {
                visit(g, neigh, path, name_of, counts, joker_taken);
            } else if !joker_taken && can_visit_small_with_joker(neigh, path, name_of) {
                visit(g, neigh, path, name_of, counts, true);
            }
        }
        path.pop();
    }

    let mut counts = vec![0; name_of.len()];
    visit(
        &g,
        index_of["start"],
        &mut Vec::new(),
        &name_of,
        &mut counts,
        false, // Part 1: true ; Part 2: false
    );

    println!("{}", counts[index_of["end"]]);
}
