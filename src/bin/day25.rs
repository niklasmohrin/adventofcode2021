use std::{collections::HashSet, io};

type Position = (usize, usize);

struct State {
    is_blocked: Vec<Vec<bool>>,
    east_positions: HashSet<Position>,
    south_positions: HashSet<Position>,
}

impl State {
    fn new(map: &str) -> Self {
        let mut is_blocked = Vec::new();
        let mut east_positions = HashSet::new();
        let mut south_positions = HashSet::new();

        for (y, line) in map.lines().enumerate() {
            is_blocked.push(vec![false; line.len()]);
            for (x, c) in line.chars().enumerate() {
                match c {
                    '>' => {
                        is_blocked[y][x] = true;
                        east_positions.insert((x, y));
                    }
                    'v' => {
                        is_blocked[y][x] = true;
                        south_positions.insert((x, y));
                    }
                    _ => (),
                }
            }
        }

        Self {
            is_blocked,
            east_positions,
            south_positions,
        }
    }

    fn move_direction(
        is_blocked: &mut Vec<Vec<bool>>,
        positions: &mut HashSet<Position>,
        direction: (usize, usize),
    ) -> bool {
        let mut will_move = Vec::new();
        for &position in positions.iter() {
            let ny = (position.1 + direction.1) % is_blocked.len();
            let nx = (position.0 + direction.0) % is_blocked[ny].len();
            if !is_blocked[ny][nx] {
                will_move.push(position);
            }
        }

        for position in will_move.iter() {
            let ny = (position.1 + direction.1) % is_blocked.len();
            let nx = (position.0 + direction.0) % is_blocked[ny].len();
            is_blocked[position.1][position.0] = false;
            is_blocked[ny][nx] = true;
            positions.remove(position);
            positions.insert((nx, ny));
        }

        !will_move.is_empty()
    }

    fn step(&mut self) -> bool {
        let east_move =
            Self::move_direction(&mut self.is_blocked, &mut self.east_positions, (1, 0));
        let south_move =
            Self::move_direction(&mut self.is_blocked, &mut self.south_positions, (0, 1));
        east_move || south_move
    }
}

fn main() {
    let input = io::read_to_string(io::stdin().lock()).expect("Failed to read from stdin.");
    let mut state = State::new(&input);
    for i in 1.. {
        if !state.step() {
            dbg!(i);
            break;
        }
    }
}
