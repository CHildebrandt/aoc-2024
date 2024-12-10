use grid::{Grid, Position};
use pathfinding::prelude::astar;
use std::collections::HashSet;

use utils::*;

const TEST: &str = include_str!("./input/test.txt");
const INPUT: &str = include_str!("./input/input.txt");

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input, |c| c.to_digit(10).unwrap() as usize);
    let starting_positions = grid.get_positions_where(|v| *v == 0);
    let ending_positions = grid.get_positions_where(|v| *v == 9);
    starting_positions
        .iter()
        .fold(
            HashSet::<(Position, usize)>::new(),
            |mut successes, start| {
                let search_in = ending_positions
                    .iter()
                    .filter(|end| !successes.iter().any(|(success_pos, _)| success_pos == *end))
                    .map(|x| *x)
                    .collect::<Vec<_>>();
                if search_in.is_empty() {
                    return successes;
                }
                successes.extend(search_in.iter().filter_map(|end| {
                    if let Some((_, score)) = astar(
                        start,
                        |a| {
                            grid.neighbors_cardinal(*a)
                                .iter()
                                .filter(|b| {
                                    grid.get(*a).unwrap().abs_diff(*grid.get(**b).unwrap()) == 1
                                })
                                .map(|a| (*a, 1))
                                .collect::<Vec<_>>()
                        },
                        |a| grid.distance_cardinal(*a, *end),
                        |x| *x == *end,
                    ) {
                        Some((*end, score))
                    } else {
                        None
                    }
                }));
                successes
            },
        )
        .iter()
        .fold(0, |acc, (_, score)| acc + score)
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    test_part1(|| part1(TEST), 36);
    // answer_part1(|| part1(INPUT), 0);
    // test_part2(|| part2(TEST), 0);
    // answer_part2(|| part2(INPUT), 0);
}
