#![feature(generic_const_exprs)]

use std::{
    collections::{BinaryHeap, HashMap},
    fmt::Display,
    io,
};

type Position = (i8, i8);
type Cost = u32;
const FLOOR_POSITIONS: [Position; 7] = [(1, 1), (2, 1), (4, 1), (6, 1), (8, 1), (10, 1), (11, 1)];
const FLOOR_Y: i8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State<const PLAYERS_PER_COLOR: usize> {
    positions: [[Position; PLAYERS_PER_COLOR]; 4],
}

const TARGET_STATE_2: State<2> = State {
    positions: [
        [(3, 2), (3, 3)],
        [(5, 2), (5, 3)],
        [(7, 2), (7, 3)],
        [(9, 2), (9, 3)],
    ],
};
const TARGET_STATE_4: State<4> = State {
    positions: [
        [(3, 2), (3, 3), (3, 4), (3, 5)],
        [(5, 2), (5, 3), (5, 4), (5, 5)],
        [(7, 2), (7, 3), (7, 4), (7, 5)],
        [(9, 2), (9, 3), (9, 4), (9, 5)],
    ],
};

impl<const PLAYERS_PER_COLOR: usize> State<PLAYERS_PER_COLOR>
where
    [(); 4 * PLAYERS_PER_COLOR]: Default,
{
    fn all_taken_positions(&self) -> [Position; 4 * PLAYERS_PER_COLOR] {
        std::array::from_fn(|i| self.positions[i % 4][i / 4])
    }

    fn target_x_for_color(color: usize) -> i8 {
        match color {
            0 => 3,
            1 => 5,
            2 => 7,
            3 => 9,
            _ => unreachable!(),
        }
    }
    fn cost_for_color(color: usize) -> Cost {
        match color {
            0 => 1,
            1 => 10,
            2 => 100,
            3 => 1000,
            _ => unreachable!(),
        }
    }

    fn with_move(mut self, color: usize, player_index: usize, target_position: Position) -> Self {
        self.positions[color][player_index] = target_position;
        self.positions[color].sort_unstable();
        self
    }

    fn cost_for(color: usize, p1: Position, p2: Position) -> Cost {
        Self::cost_for_color(color) * ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as Cost
    }

    fn target_position_for(&self, color: usize) -> Option<Position> {
        let target_x = Self::target_x_for_color(color);
        (!self.has_other_colors_in_room(color)).then(|| {
            let y = self.positions[color]
                .iter()
                .filter(|p| p.0 == target_x)
                .map(|p| p.1)
                .min()
                .unwrap_or((PLAYERS_PER_COLOR + 2) as i8)
                - 1;
            (target_x, y)
        })
    }

    fn path_is_free(&self, mut start: Position, end: Position) -> bool {
        let blocked_positions = self.all_taken_positions();

        let move_x = |start: &mut Position| {
            let dx = if start.0 < end.0 { 1 } else { -1 };
            while start.0 != end.0 {
                start.0 += dx;
                if blocked_positions.contains(&start) {
                    return false;
                }
            }
            true
        };
        let move_y = |start: &mut Position| {
            let dy = if start.1 < end.1 { 1 } else { -1 };
            while start.1 != end.1 {
                start.1 += dy;
                if blocked_positions.contains(&start) {
                    return false;
                }
            }
            true
        };

        if start.1 == FLOOR_Y {
            move_x(&mut start) && move_y(&mut start)
        } else {
            move_y(&mut start) && move_x(&mut start)
        }
    }

    fn has_other_colors_in_room(&self, color: usize) -> bool {
        self.positions
            .iter()
            .enumerate()
            .any(|(other_color, positions)| {
                color != other_color
                    && positions
                        .iter()
                        .any(|p| p.0 == Self::target_x_for_color(color))
            })
    }

    fn should_move_to_floor(&self, color: usize, player_index: usize) -> bool {
        let position = self.positions[color][player_index];
        if position.1 == FLOOR_Y {
            return false;
        }

        if position.0 != Self::target_x_for_color(color) {
            return true;
        }

        self.has_other_colors_in_room(color)
    }

    fn for_each_next_possible_state(&self, mut f: impl FnMut(Self, Cost)) {
        'floor_loop: for floor_position in FLOOR_POSITIONS {
            for (color, player_positions) in self.positions.iter().enumerate() {
                for (player_index, &position) in player_positions.iter().enumerate() {
                    if position == floor_position {
                        // A player is on here, check if they can move to their target position.
                        if let Some(target_position) = self.target_position_for(color) {
                            if self.path_is_free(position, target_position) {
                                f(
                                    self.with_move(color, player_index, target_position),
                                    Self::cost_for(color, position, target_position),
                                );
                            }
                        }

                        // No other player can move to this position anyways, because the player
                        // blocks it.
                        continue 'floor_loop;
                    }
                }
            }

            // No player is on the floor position, so check who can move here.

            for (color, player_positions) in self.positions.iter().enumerate() {
                for (player_index, &position) in player_positions.iter().enumerate() {
                    if self.should_move_to_floor(color, player_index)
                        && self.path_is_free(position, floor_position)
                    {
                        f(
                            self.with_move(color, player_index, floor_position),
                            Self::cost_for(color, position, floor_position),
                        );
                    }
                }
            }
        }
    }
}

impl Display for State<2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut field = [
            *b"#############",
            *b"#...........#",
            *b"###.#.#.#.###",
            *b"  #.#.#.#.#  ",
            *b"  #########  ",
        ];
        for (color, player_positions) in self.positions.iter().enumerate() {
            for position in player_positions {
                field[position.1 as usize][position.0 as usize] = b'A' + color as u8;
            }
        }
        for line in field {
            writeln!(f, "{}", std::str::from_utf8(&line).unwrap())?;
        }
        Ok(())
    }
}
impl Display for State<4> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut field = [
            *b"#############",
            *b"#...........#",
            *b"###.#.#.#.###",
            *b"  #.#.#.#.#  ",
            *b"  #.#.#.#.#  ",
            *b"  #.#.#.#.#  ",
            *b"  #########  ",
        ];
        for (color, player_positions) in self.positions.iter().enumerate() {
            for position in player_positions {
                field[position.1 as usize][position.0 as usize] = b'A' + color as u8;
            }
        }
        for line in field {
            writeln!(f, "{}", std::str::from_utf8(&line).unwrap())?;
        }
        Ok(())
    }
}

fn main() {
    let input = io::read_to_string(io::stdin().lock()).expect("Failed to read stdin.");

    // Change for Part 1 or 2
    const PLAYERS_PER_COLOR: usize = 4;
    const TARGET_STATE: State<PLAYERS_PER_COLOR> = TARGET_STATE_4;

    let mut next_index = [0; 4];
    let player_order = "ABCD";
    let mut initial_positions = [[(0, 0); PLAYERS_PER_COLOR]; 4];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if let Some(color) = player_order.chars().position(|c2| c2 == c) {
                initial_positions[color][next_index[color]] = (x as i8, y as i8);
                next_index[color] += 1;
            }
        }
    }

    assert_eq!(next_index, [PLAYERS_PER_COLOR; 4]);

    let initial_state = State {
        positions: initial_positions,
    };

    eprintln!("{initial_state}");

    #[derive(PartialEq, Eq)]
    struct Item {
        cost: Cost,
        state: State<PLAYERS_PER_COLOR>,
    }

    impl PartialOrd for Item {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Item {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    let mut heap = BinaryHeap::<Item>::new();
    heap.push(Item {
        cost: 0,
        state: initial_state,
    });
    let mut best_cost = HashMap::new();

    while let Some(Item { cost, state }) = heap.pop() {
        if let Some((other_cost, _)) = best_cost.get(&state) {
            if cost > *other_cost {
                continue;
            }
        }

        state.for_each_next_possible_state(|next_state, additional_cost| {
            let next_cost = cost + additional_cost;
            if let Some((other_cost, _)) = best_cost.get(&next_state) {
                if *other_cost <= next_cost {
                    return;
                }
            }
            best_cost.insert(next_state, (next_cost, state));
            heap.push(Item {
                cost: next_cost,
                state: next_state,
            });
        })
    }

    eprintln!("Moves:");
    let mut states = vec![TARGET_STATE];
    while states.last().unwrap() != &initial_state {
        let prev = best_cost
            .get(states.last().unwrap())
            .expect("No predecessor!")
            .1;
        states.push(prev);
    }

    for state in states.iter().rev() {
        eprintln!("{}", state);
    }

    println!("Cost: {}", best_cost.get(&TARGET_STATE).unwrap().0);
}
