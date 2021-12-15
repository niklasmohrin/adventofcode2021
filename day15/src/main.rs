use std::{
    cmp,
    collections::BinaryHeap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let node_costs: Vec<Vec<_>> = stdin
        .lock()
        .lines()
        .map(|row| {
            row.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let n = node_costs.len();
    let actual_n = 5 * n; // Part 1: n; Part 2: 5 * n

    let calculated_cost = |x: usize, y: usize| {
        let block_x = x / n;
        let block_y = y / n;
        let additonal_cost = block_x + block_y;
        ((node_costs[y % n][x % n] + additonal_cost as u32) - 1) % 9 + 1
    };

    fn neighs(x: usize, y: usize, max_val: usize) -> impl IntoIterator<Item = (usize, usize)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .filter_map(move |(dx, dy)| {
                ((x as isize) + dx)
                    .try_into()
                    .ok()
                    .zip(((y as isize) + dy).try_into().ok())
            })
            .filter(move |&(x, y)| x < max_val && y < max_val)
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Relaxation {
        node: (usize, usize),
        cost: u32,
    }
    impl Ord for Relaxation {
        fn cmp(&self, other: &Self) -> cmp::Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.node.cmp(&other.node))
        }
    }
    impl PartialOrd for Relaxation {
        fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut shortest_known: Vec<Vec<_>> = (0..actual_n)
        .map(|_| (0..actual_n).map(|_| u32::MAX).collect())
        .collect();
    let mut queue = BinaryHeap::new();
    shortest_known[0][0] = 0;
    queue.push(Relaxation {
        node: (0, 0),
        cost: 0,
    });

    while let Some(Relaxation { node: (x, y), cost }) = queue.pop() {
        if cost > shortest_known[y][x] {
            continue;
        }

        for (nx, ny) in neighs(x, y, actual_n) {
            let next = Relaxation {
                cost: cost + calculated_cost(nx, ny),
                node: (nx, ny),
            };
            if next.cost < shortest_known[ny][nx] {
                shortest_known[ny][nx] = next.cost;
                queue.push(next);
            }
        }
    }

    println!("{}", shortest_known[actual_n-1][actual_n-1]);
}
