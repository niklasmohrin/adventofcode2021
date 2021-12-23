use std::{
    io::{self, BufRead},
    ops::RangeInclusive,
};

use itertools::Itertools;
use serde_scan::scan;

fn main() {
    let stdin = io::stdin();
    let line = &stdin.lock().lines().next().unwrap().unwrap();
    let (target_xs, target_ys): (RangeInclusive<i64>, RangeInclusive<i64>) =
        scan!("target area: x={}..{}, y={}..{}" <- line).unwrap();

    let steps_in_target =
        |mut vel: i64, target: &RangeInclusive<i64>, keep_decreasing_vel: bool| {
            const MAX_STEPS: i64 = 1000;

            let mut pos = 0;
            let mut inside = Vec::new();
            let mut max_pos = 0;
            for i in 0..MAX_STEPS {
                max_pos = max_pos.max(pos);

                if pos < *target.start() && vel <= 0 {
                    break;
                }

                if target.contains(&pos) {
                    inside.push(i);
                }

                pos += vel;
                vel -= 1;
                if !keep_decreasing_vel {
                    vel = vel.max(0);
                }
            }

            (inside, max_pos)
        };

    let ys = (-500..500).map(|vy_init| (vy_init, steps_in_target(vy_init, &target_ys, true)));
    let xs = (0..1000).map(|vx_init| (vx_init, steps_in_target(vx_init, &target_xs, false)));
    let combinations = ys.cartesian_product(xs).filter(
        |((_vy_init, (vy_steps, _max_y)), (_vx_init, (vx_steps, _max_x)))| {
            vy_steps.iter().any(|s| vx_steps.contains(s))
        },
    );

    // Part 1
    // println!(
    //     "{}",
    //     combinations
    //         .max_by_key(|((_vy_init, (_vy_steps, max_y)), _)| *max_y)
    //         .unwrap()
    //         .0
    //          .1
    //          .1
    // );

    // Part 2
    println!("{}", combinations.count());
}
