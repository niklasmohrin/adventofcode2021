use std::{
    collections::HashSet,
    io::{self, Read},
};

use itertools::Itertools;

struct Canvas {
    background_lit: bool,
    exact_bounds: [i32; 4],
    lit_in_bounds: HashSet<(i32, i32)>,
    enhancement_table: Vec<bool>,
}

impl Canvas {
    pub fn from_input(input: &str) -> Self {
        let (enhancement_table, image) = input.split_once("\n\n").unwrap();
        let enhancement_table: Vec<bool> = enhancement_table.chars().map(|c| c == '#').collect();
        let lit_in_bounds = Self::parse_input_image(image);
        let exact_bounds = Self::minimal_bounds(&lit_in_bounds);
        Self {
            enhancement_table,
            lit_in_bounds,
            background_lit: false,
            exact_bounds,
        }
    }

    fn parse_input_image(image: &str) -> HashSet<(i32, i32)> {
        image
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .map(|c| c == '#')
                    .enumerate()
                    .filter_map(move |(x, b)| b.then(|| (x as _, y as _)))
            })
            .collect()
    }

    fn minimal_bounds(points: &HashSet<(i32, i32)>) -> [i32; 4] {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;

        for &(x, y) in points.iter() {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }

        [min_x, max_x, min_y, max_y]
    }

    fn is_in_exact_bounds(&self, x: i32, y: i32) -> bool {
        let [min_x, max_x, min_y, max_y] = self.exact_bounds;
        min_x <= x && x <= max_x && min_y <= y && y <= max_y
    }

    fn index_for(&self, x: i32, y: i32) -> usize {
        let mut index = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = x + dx;
                let ny = y + dy;
                let val = if self.is_in_exact_bounds(nx, ny) {
                    self.lit_in_bounds.contains(&(nx, ny))
                } else {
                    self.background_lit
                };
                index <<= 1;
                index += val as usize;
            }
        }
        index
    }

    fn should_be_lit(&self, x: i32, y: i32) -> bool {
        self.enhancement_table[self.index_for(x, y)]
    }

    fn enhance(&mut self) {
        let new_exact_bounds = [
            self.exact_bounds[0] - 2,
            self.exact_bounds[1] + 2,
            self.exact_bounds[2] - 2,
            self.exact_bounds[3] + 2,
        ];

        self.update_exact_area(new_exact_bounds);
        self.exact_bounds = new_exact_bounds;
        self.background_lit =
            self.enhancement_table[if self.background_lit { 0b111111111 } else { 0 }];
    }

    fn update_exact_area(&mut self, [min_x, max_x, min_y, max_y]: [i32; 4]) {
        self.lit_in_bounds = (min_x..=max_x)
            .cartesian_product(min_y..=max_y)
            .filter(|&(x, y)| self.should_be_lit(x, y))
            .collect()
    }

    pub fn lit_count(&self) -> usize {
        assert!(!self.background_lit);
        self.lit_in_bounds.len()
    }
}

fn main() {
    let input = {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        s
    };

    let mut canvas = Canvas::from_input(&input);
    for _ in 0..50 {
        canvas.enhance();
    }
    dbg!(canvas.lit_count());
}
