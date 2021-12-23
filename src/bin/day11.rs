use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut board = [[0u8; 10]; 10];
    for (y, line) in stdin.lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            board[y][x] = c.to_digit(10).unwrap() as _;
        }
    }

    let mut total_flashes = 0;
    let mut step = || {
        let increment =
            |x: usize, y: usize, board: &mut [[u8; 10]; 10], left_to_flash: &mut Vec<(i8, i8)>| {
                board[y][x] += 1;
                if board[y][x] > 9 {
                    left_to_flash.push((x as _, y as _));
                    board[y][x] = 0;
                }
            };

        let mut flashed_this_step = 0;
        let mut left_to_flash = Vec::new();

        (0..10).for_each(|x| (0..10).for_each(|y| increment(x, y, &mut board, &mut left_to_flash)));

        while let Some((x, y)) = left_to_flash.pop() {
            flashed_this_step += 1;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    match ((x + dx).try_into(), (y + dy).try_into()) {
                        (Ok(nx @ 0..=9), Ok(ny @ 0..=9)) if board[ny][nx] > 0 => {
                            increment(nx, ny, &mut board, &mut left_to_flash);
                        }
                        _ => {}
                    }
                }
            }
        }

        total_flashes += flashed_this_step;
        flashed_this_step == 100
    };

    // Part 1
    // (0..100).for_each(|_| step());
    // dbg!(total_flashes);

    // Part 2
    let first_time_all_flash = (1..).find(|_| step()).unwrap();
    dbg!(first_time_all_flash);
}
