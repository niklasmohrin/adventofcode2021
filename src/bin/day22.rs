use std::{
    cmp,
    collections::HashSet,
    hash::Hash,
    io::{self, BufRead},
    mem,
    ops::{Range, Sub},
};

use itertools::Itertools;

const DIMENSIONS: usize = 3;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Cube {
    dimensions: [Range<i32>; DIMENSIONS],
}

impl Cube {
    fn intersect(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (dim, other_dim) in result.dimensions.iter_mut().zip(&other.dimensions) {
            dim.start = cmp::max(dim.start, other_dim.start);
            dim.end = cmp::max(dim.start, cmp::min(dim.end, other_dim.end));
        }
        result
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Command {
    cube: Cube,
    state: bool,
    priority: usize,
}

fn scanline<Obj: Copy + Eq + Hash, Time: Copy + Sub + Eq + Ord>(
    objects: impl IntoIterator<Item = Obj>,
    interval: impl Fn(Obj) -> Range<Time>,
    mut emit_active: impl FnMut(&HashSet<Obj>, <Time as Sub>::Output),
) {
    let grouped_events = objects
        .into_iter()
        .flat_map(|obj| {
            let int = interval(obj);
            [(int.start, false, obj), (int.end, true, obj)]
        })
        .sorted_unstable_by_key(|(time, is_end, _)| (*time, *is_end))
        .group_by(|(time, is_end, _)| (*time, *is_end));
    let mut grouped_events = grouped_events.into_iter();

    let Some(((first_event_time, first_event_is_end), first_event_values)) = grouped_events.next() else {
        return;
    };
    let mut prev_time = first_event_time;
    assert!(!first_event_is_end);
    let mut active = HashSet::from_iter(first_event_values.map(|(_, _, obj)| obj));

    for ((time, is_end), events) in grouped_events {
        emit_active(&active, time - mem::replace(&mut prev_time, time));

        if is_end {
            for (_, _, obj) in events {
                assert!(active.remove(&obj));
            }
        } else {
            for (_, _, obj) in events {
                assert!(active.insert(obj));
            }
        }
    }
}

fn scanline3<'a>(commands: impl IntoIterator<Item = &'a Command>) -> u64 {
    let mut total = 0;

    scanline(
        commands,
        |cmd| cmd.cube.dimensions[0].clone(),
        |active_x, len_x| {
            scanline(
                active_x.iter(),
                |cmd| cmd.cube.dimensions[1].clone(),
                |active_y, len_y| {
                    scanline(
                        active_y.iter(),
                        |cmd| cmd.cube.dimensions[2].clone(),
                        |active_z, len_z| {
                            if let Some(top_command) =
                                active_z.iter().max_by_key(|cmd| cmd.priority)
                            {
                                total += top_command.state as u64
                                    * len_x as u64
                                    * len_y as u64
                                    * len_z as u64;
                            }
                        },
                    );
                },
            );
        },
    );

    total
}

fn main() {
    let mut commands = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let (state, cube_data) = line.split_once(" ").unwrap();
        let parse_range = |s: &str| {
            let (start, end) = s.split_once("..").unwrap();
            start.parse::<i32>().unwrap()..end.parse::<i32>().unwrap() + 1
        };
        let mut dimensions = cube_data.split(",");
        let cube = Cube {
            dimensions: [
                parse_range(dimensions.next().unwrap().strip_prefix("x=").unwrap()),
                parse_range(dimensions.next().unwrap().strip_prefix("y=").unwrap()),
                parse_range(dimensions.next().unwrap().strip_prefix("z=").unwrap()),
            ],
        };
        commands.push(Command {
            cube,
            state: state == "on",
            priority: commands.len(),
        });
    }

    let restricted_commands = commands
        .iter()
        .map(|cmd| Command {
            cube: cmd.cube.intersect(&Cube {
                dimensions: [-50..51, -50..51, -50..51],
            }),
            ..cmd.clone()
        })
        .collect_vec();
    dbg!(scanline3(&restricted_commands));
    dbg!(scanline3(&commands));
}
