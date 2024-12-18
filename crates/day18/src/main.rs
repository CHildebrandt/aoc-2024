use grid::GridPos;
use itertools::Itertools;
use pathfinding::prelude::astar;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Memory {
    Safe,
    Corrupt,
}

fn part1(input: &str, size: usize, wait: usize) -> usize {
    let start = (0, 0);
    let end = (size - 1, size - 1);
    let positions = input
        .lines()
        .map(|line| GridPos::<usize>::from_str(line).unwrap().flip())
        .collect_vec();
    let mut grid = grid::Grid::blank(size, size, Memory::Safe);
    positions.iter().take(wait).for_each(|pos| {
        grid[pos] = Memory::Corrupt;
    });
    astar(
        &start,
        |pos| {
            grid.neighbors_cardinal(*pos)
                .iter()
                .filter_map(|pos| {
                    if grid.get(*pos).unwrap() == &Memory::Corrupt {
                        None
                    } else {
                        Some((*pos, 1))
                    }
                })
                .collect_vec()
        },
        |a| grid.distance_cardinal(*a, end),
        |pos| pos == &end,
    )
    .unwrap()
    .1
}

fn part2(input: &str, size: usize) -> (usize, usize) {
    let start = (0, 0);
    let end = (size - 1, size - 1);
    let positions = input
        .lines()
        .map(|line| GridPos::<usize>::from_str(line).unwrap().flip())
        .collect_vec();
    let grid = grid::Grid::blank(size, size, Memory::Safe);
    let (_, found) = positions
        .iter()
        .enumerate()
        .find(|(i, _)| {
            let mut grid = grid.clone();
            positions.iter().take(i + 1).for_each(|pos| {
                grid[pos] = Memory::Corrupt;
            });
            astar(
                &start,
                |pos| {
                    grid.neighbors_cardinal(*pos)
                        .iter()
                        .filter_map(|pos| {
                            if grid.get(*pos).unwrap() == &Memory::Corrupt {
                                None
                            } else {
                                Some((*pos, 1))
                            }
                        })
                        .collect_vec()
                },
                |a| grid.distance_cardinal(*a, end),
                |pos| pos == &end,
            )
            .is_none()
        })
        .unwrap();
    (found.1, found.0)
}

fn main() {
    test_part1(|| part1(include_str!("./input/test.txt"), 7, 12), 22);
    answer_part1(|| part1(include_str!("./input/input.txt"), 71, 1024), 272);
    test_part2(|| part2(include_str!("./input/test.txt"), 7), (6, 1));
    test_part2(|| part2(include_str!("./input/input.txt"), 71), (16, 44));
}
