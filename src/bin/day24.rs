use std::{
    cmp,
    collections::{hash_map::Entry, HashMap, VecDeque},
};

use itertools::Itertools;

/*
 * Notes: The program matches 14 repetitions of the following regex:
 *
 *   inp w
 *   mul x 0
 *   add x z
 *   mod x 26
 *   div z (1|26)
 *   add x (-?\d\d?)
 *   eql x w
 *   eql x 0
 *   mul y 0
 *   add y 25
 *   mul y x
 *   add y 1
 *   mul z y
 *   mul y 0
 *   add y w
 *   add y (\d\d?)
 *   mul y x
 *   add z y
 *
 * There are only three parameters for each input (the three capture groups of the regex).
 *
 * Another observation: Since modulo is only allowed with non-negative left-hand-side, z must stay
 * non-negative throughout the process.
 * */

type Int = i64;

fn round(mut z: Int, w: Int, p1: Int, p2: Int, p3: bool) -> Int {
    let x = (z % 26) + p1;
    if p3 {
        z /= 26;
    }
    if x != w {
        z *= 26;
        z += w + p2;
    }
    z
}

fn inverse_round(
    mut z: Int,
    guessed_final_remainder: Int,
    w: Int,
    p1: Int,
    p2: Int,
    p3: bool,
) -> Option<Int> {
    let x = guessed_final_remainder + p1;
    if x != w {
        z -= w + p2;
        if z % 26 != 0 || z < 0 {
            return None;
        }
        z /= 26;
    }
    if !p3 {
        (z % 26 == guessed_final_remainder).then_some(z)
    } else {
        Some(z * 26 + guessed_final_remainder)
    }
}

/// Parameters in my input.
const PARAMETERS: [(Int, Int, bool); 14] = [
    (10, 2, false),
    (15, 16, false),
    (14, 9, false),
    (15, 0, false),
    (-8, 1, true),
    (10, 12, false),
    (-16, 6, true),
    (-4, 6, true),
    (11, 3, false),
    (-3, 5, true),
    (12, 9, false),
    (-7, 3, true),
    (-15, 2, true),
    (-7, 3, true),
];

fn crack_forwards() {
    let mut z_values = vec![HashMap::new()];
    z_values[0].insert(0, Vec::new());

    for (p1, p2, p3) in PARAMETERS {
        let mut new_values = HashMap::<Int, Vec<(Int, Int)>>::new();
        z_values
            .last()
            .unwrap()
            .keys()
            .cartesian_product(1..=9)
            .for_each(|(z, w)| {
                let next_z = round(*z, w, p1, p2, p3);
                if 0 <= next_z {
                    new_values.entry(next_z).or_default().push((*z, w));
                }
            });
        z_values.push(new_values);
        dbg!(z_values.last().unwrap().len());
    }

    // reconstruct
    let mut valid_passcodes = Vec::new();
    let mut stack = vec![(0, Vec::new())];
    while !stack.is_empty() {
        let (last_z, digits_so_far) = stack.pop().unwrap();
        if digits_so_far.len() == 14 {
            let mut num = 0;
            for &d in digits_so_far.iter().rev() {
                num *= 10;
                num += d;
            }
            valid_passcodes.push(num);

            continue;
        }

        for (z, w) in z_values[z_values.len() - 1 - digits_so_far.len()]
            .get(&last_z)
            .unwrap()
        {
            let mut new_digits = digits_so_far.clone();
            new_digits.push(w);
            stack.push((*z, new_digits));
        }
    }

    dbg!(valid_passcodes.len());
    dbg!(valid_passcodes.iter().min());
    dbg!(valid_passcodes.iter().max());
}

fn crack_backwards() {
    let mut valid_passcodes = Vec::new();

    let mut best_digits = <[HashMap<Int, (Int, Int)>; PARAMETERS.len() + 1]>::default();
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    best_digits[0].insert(0, (0, 0));

    let mut seen_depths = 0;

    while !queue.is_empty() {
        let (last_z, digit_count) = queue.pop_front().unwrap();
        if digit_count > seen_depths {
            dbg!(digit_count, queue.len());
            seen_depths = digit_count;
        }
        let &(min_digits_so_far, max_digits_so_far) =
            best_digits[digit_count].get(&last_z).unwrap();
        if digit_count == PARAMETERS.len() {
            valid_passcodes.push(min_digits_so_far);
            valid_passcodes.push(max_digits_so_far);
            continue;
        }

        let round_index = PARAMETERS.len() - 1 - digit_count;
        let (p1, p2, p3) = PARAMETERS[round_index];

        for w in 1..=9 {
            for guessed_final_remainder in 0..26 {
                if let Some(z) = inverse_round(last_z, guessed_final_remainder, w, p1, p2, p3) {
                    let new_min_digits = w * (10 as Int).pow(digit_count as _) + min_digits_so_far;
                    let new_max_digits = w * (10 as Int).pow(digit_count as _) + max_digits_so_far;

                    match best_digits[digit_count + 1].entry(z) {
                        Entry::Occupied(mut entry) => {
                            let digits = entry.get_mut();
                            digits.0 = cmp::min(digits.0, new_min_digits);
                            digits.1 = cmp::max(digits.1, new_max_digits);
                        }
                        Entry::Vacant(entry) => {
                            entry.insert((new_min_digits, new_max_digits));
                        }
                    };

                    queue.push_back((z, digit_count + 1));
                }
            }
        }
    }

    dbg!(valid_passcodes.len());
    dbg!(valid_passcodes.iter().min());
    dbg!(valid_passcodes.iter().max());
}

fn main() {
    crack_forwards();
    crack_backwards();
}
