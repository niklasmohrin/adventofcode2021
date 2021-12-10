use std::io::{self, BufRead};
use std::str::FromStr;

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let initial_timer_values = line.split(',').map(usize::from_str).map(Result::unwrap);

    let mut timer_value_count = [0; 10];
    for time_left in initial_timer_values {
        timer_value_count[time_left] += 1;
    }

    let index_for = |timer, shift| (timer + shift) % 10;
    for time_shift in 0..256usize {
        timer_value_count[index_for(9, time_shift)] += timer_value_count[index_for(0, time_shift)];
        timer_value_count[index_for(7, time_shift)] += timer_value_count[index_for(0, time_shift)];
        timer_value_count[index_for(0, time_shift)] = 0;
    }
    println!("{}", timer_value_count.iter().sum::<usize>());
}
