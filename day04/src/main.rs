use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
    str::FromStr,
};

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut lines = stdin.lines();
    let picks = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(usize::from_str)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let mut boards = Vec::new();
    let mut board_index = HashMap::new();
    while let Some(_empty_line) = lines.next() {
        let rows = (0..5)
            .map(|_| {
                lines
                    .next()
                    .unwrap()
                    .unwrap()
                    .split_whitespace()
                    .map(usize::from_str)
                    .map(Result::unwrap)
                    .inspect(|&num| {
                        board_index
                            .entry(num)
                            .or_insert_with(Vec::new)
                            .push(boards.len())
                    })
                    .collect()
            })
            .collect::<Vec<Vec<_>>>();
        boards.push(rows);
    }

    let has_won = |board: &Vec<Vec<usize>>, marked: &HashSet<usize>| {
        (0..5).any(|i| {
            (0..5).all(|j| marked.contains(&board[i][j]))
                || (0..5).all(|j| marked.contains(&board[j][i]))
        })
    };

    let score = |board: &Vec<Vec<usize>>, marked: &HashSet<usize>| {
        board
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|num| !marked.contains(num))
                    .sum::<usize>()
            })
            .sum::<usize>()
    };

    let mut marked = HashSet::new();
    let mut boards_still_playing = boards.iter().collect::<HashSet<_>>();
    let mut part1_sol = None;
    let mut last_winner = None;

    for pick in picks {
        if boards_still_playing.is_empty() {
            break;
        }

        marked.insert(pick);
        for &candidate in board_index[&pick].iter() {
            let b = &boards[candidate];
            if boards_still_playing.contains(b) && has_won(b, &marked) {
                if part1_sol.is_none() {
                    part1_sol = Some(score(b, &marked) * pick);
                }
                // only compute the score at the end
                let _ = last_winner.insert((b, pick));
                boards_still_playing.remove(b);
            }
        }
    }

    println!("{}", part1_sol.unwrap());
    let (loser_board, loser_pick) = last_winner.unwrap();
    let loser_score = score(loser_board, &marked);
    println!("{}", loser_score * loser_pick);
}
