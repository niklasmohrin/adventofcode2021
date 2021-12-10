use std::{
    collections::HashSet,
    io::{self, Read},
};

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn main() {
    let mut stdin = io::stdin();
    let mut raw_input = String::new();
    stdin.read_to_string(&mut raw_input).unwrap();

    let heights: Vec<Vec<u32>> = raw_input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // Part 1
    let mut total = 0;
    for y in 0..heights.len() {
        for x in 0..heights[y].len() {
            let is_low_point = DIRECTIONS
                .iter()
                .flat_map(|&(dx, dy)| {
                    (x as i32 + dx)
                        .try_into()
                        .ok()
                        .zip((y as i32 + dy).try_into().ok())
                        .and_then(|(nx, ny): (usize, usize)| {
                            heights.get(ny).and_then(|r| r.get(nx))
                        })
                })
                .all(|&nh| nh > heights[y][x]);
            if is_low_point {
                total += heights[y][x] + 1;
            }
        }
    }
    println!("{}", total);

    let mut seen = HashSet::new();
    let mut basins = Vec::new();
    for y in 0..heights.len() {
        for x in 0..heights[y].len() {
            if seen.contains(&(x, y)) || heights[y][x] == 9 {
                continue;
            }
            seen.insert((x, y));
            let mut basin_size = 0;
            let mut s = vec![(x as i32, y as i32)];
            while let Some((cur_x, cur_y)) = s.pop() {
                basin_size += 1;
                let neighs = DIRECTIONS
                    .iter()
                    .map(|(dx, dy)| (cur_x + dx, cur_y + dy))
                    .filter(|&(x, y)| {
                        0 <= y
                            && y < heights.len() as _
                            && 0 <= x
                            && x < heights[y as usize].len() as _
                    });
                for (nx, ny) in neighs {
                    if heights[ny as usize][nx as usize] != 9
                        && !seen.contains(&(nx as usize, ny as usize))
                    {
                        s.push((nx, ny));
                        seen.insert((nx as usize, ny as usize));
                    }
                }
            }
            basins.push(basin_size);
        }
    }
    basins.sort_unstable();
    basins.reverse();
    println!("{}", basins[0] * basins[1] * basins[2]);
}
