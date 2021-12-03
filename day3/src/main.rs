use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let readings = stdin
        .lines()
        .map(|line| line.unwrap().chars().map(|c| c == '1').collect())
        .collect::<Vec<Vec<bool>>>();
    let column_count = readings[0].len();

    // Part 1
    let mut column_sum = vec![0; column_count];
    for row in readings.iter() {
        for (count, &is_one) in column_sum.iter_mut().zip(row) {
            *count += is_one as usize;
        }
    }
    let one_is_most_common = column_sum
        .iter()
        .map(|&count| count > readings.len() / 2)
        .collect::<Vec<_>>();
    let gamma_rate = one_is_most_common
        .iter()
        .fold(0, |acc, &cur| 2 * acc + cur as usize);
    let epsilon_rate = (!gamma_rate) & ((1 << column_count) - 1);
    dbg!(gamma_rate, epsilon_rate);
    println!("{}", gamma_rate * epsilon_rate);

    // Part 2
    let reading_numbers = readings
        .iter()
        .map(|row| row.iter().fold(0, |acc, &cur| 2 * acc + cur as usize))
        .collect::<HashSet<_>>();
    let eliminate_until_one_left = |mut remaining: HashSet<usize>, keep_least_common: bool| {
        let mut mask = 1 << (column_count - 1);
        while remaining.len() > 1 {
            let ones_count = remaining.iter().filter(|&num| num & mask != 0).count();
            let ones_most_common = ones_count >= (remaining.len() + 1) / 2;
            remaining.retain(|&num| !(num & mask != 0) ^ ones_most_common ^ keep_least_common);
            mask >>= 1;
        }
        remaining.into_iter().next().unwrap()
    };
    let oxygen_generator_rating = eliminate_until_one_left(reading_numbers.clone(), false);
    let co2_scrubber_rating = eliminate_until_one_left(reading_numbers.clone(), true);
    dbg!(oxygen_generator_rating, co2_scrubber_rating);
    println!("{}", oxygen_generator_rating * co2_scrubber_rating);
}
