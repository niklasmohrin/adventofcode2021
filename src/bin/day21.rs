use std::{
    cmp,
    collections::HashMap,
    io::{self, BufRead},
};

use itertools::Itertools;

fn main() {
    let positions: [u8; 2] = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect_vec()
        .try_into()
        .unwrap();

    fn mod10b1(v: u8) -> u8 {
        ((v - 1) % 10) + 1
    }

    // Part 1: Just doing it
    {
        let mut positions = positions;
        let mut scores = [0; 2];
        let mut dice_rolls = 0u32;
        let mut roll = || {
            dice_rolls += 1;
            ((dice_rolls - 1) % 100) + 1
        };

        'outer: loop {
            for (p, s) in positions.iter_mut().zip(scores.iter_mut()) {
                let moved = ((roll() + roll() + roll()) % 10) as u8;
                *p = mod10b1(*p + moved);
                *s += *p as u32;
                if *s >= 1000 {
                    break 'outer;
                }
            }
        }
        let loosing_score = cmp::min(scores[0], scores[1]);
        dbg!(loosing_score * dice_rolls);
    }

    // Part 2: DP
    {
        fn subuniverse_win_counts(
            cur_positions: [u8; 2],
            cur_scores: [u8; 2],
            cache: &mut HashMap<([u8; 2], [u8; 2]), (u64, u64)>,
        ) -> (u64, u64) {
            if !cache.contains_key(&(cur_positions, cur_scores)) {
                assert!(cur_scores[0] < 21);
                assert!(cur_scores[1] < 21);

                let mut counts = (0, 0);
                for r1 in 1..=3 {
                    for r2 in 1..=3 {
                        for r3 in 1..=3 {
                            let moved = r1 + r2 + r3;
                            let new_pos = mod10b1(cur_positions[0] + moved);
                            let new_score = cur_scores[0] + new_pos;
                            if new_score >= 21 {
                                counts.0 += 1;
                            } else {
                                let res = subuniverse_win_counts(
                                    [cur_positions[1], new_pos],
                                    [cur_scores[1], new_score],
                                    cache,
                                );
                                counts.0 += res.1;
                                counts.1 += res.0;
                            }
                        }
                    }
                }

                cache.insert((cur_positions, cur_scores), counts);
            }

            *cache.get(&(cur_positions, cur_scores)).unwrap()
        }

        let (p1, p2) = subuniverse_win_counts(positions, [0; 2], &mut HashMap::new());
        dbg!(p1, p2);
        dbg!(cmp::max(p1, p2));
    }
}
