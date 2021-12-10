use std::{
    collections::HashMap,
    io::{self, Read},
};

use itertools::Itertools;

const SEGMENTS_USED: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

fn main() {
    let raw_input = {
        let mut s = String::new();
        let _ = io::stdin().read_to_string(&mut s);
        s
    };
    let tasks = raw_input
        .lines()
        .map(|line| {
            let (seen, question) = line.split_once(" | ").unwrap();
            let seen = seen.split_whitespace().collect::<Vec<_>>();
            let question = question.split_whitespace().collect::<Vec<_>>();
            (seen, question)
        })
        .collect::<Vec<_>>();

    // Part 1
    let unqiue_digits: HashMap<usize, &'static str> = (0..10)
        .map(|count| {
            (SEGMENTS_USED.iter().filter(|s| s.len() == count).count() == 1).then(|| {
                (
                    count,
                    *SEGMENTS_USED.iter().find(|s| s.len() == count).unwrap(),
                )
            })
        })
        .flatten()
        .collect();
    let easy_count: usize = tasks
        .iter()
        .map(|(_, q)| {
            q.iter()
                .filter(|s| unqiue_digits.contains_key(&s.len()))
                .count()
        })
        .sum();
    println!("{}", easy_count);

    // Part 2
    let segments_to_mask = |segment: &str| b"abcdefg".map(|c| segment.contains(c as char));
    let correct_masks = SEGMENTS_USED.map(segments_to_mask);

    let crack = |given: &Vec<&str>| {
        let is_correct = |perm: &[usize; 7]| {
            let permuted_correct_masks = correct_masks.map(|mask| perm.map(|i| mask[i]));
            let fits_perm = |reading: &&str| {
                let reading_mask = segments_to_mask(reading);
                permuted_correct_masks
                    .iter()
                    .any(|&mask| mask == reading_mask)
            };
            given.iter().all(fits_perm)
        };
        (0..7)
            .permutations(7)
            .map(|v| v.try_into().unwrap())
            .find(is_correct)
            .unwrap()
    };

    let mut total = 0;
    for (given, question) in tasks {
        let perm = crack(&given);
        let permuted_correct_masks = correct_masks.map(|mask| perm.map(|i| mask[i]));
        let mut num = 0;
        for display in question {
            let display_mask = segments_to_mask(display);
            let digit = permuted_correct_masks
                .iter()
                .enumerate()
                .find(|(_, m)| **m == display_mask)
                .unwrap()
                .0;
            num *= 10;
            num += digit;
        }
        total += num;
    }
    println!("{}", total);
}
