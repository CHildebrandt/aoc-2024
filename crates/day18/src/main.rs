use grid::GridPos;
use itertools::Itertools;
use pathfinding::prelude::astar;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Memory {
    Safe,
    Corrupt,
}

fn parse(input: &str) -> Vec<GridPos<usize>> {
    input
        .lines()
        .map(|line| GridPos::<usize>::from_str(line).unwrap().flip())
        .collect_vec()
}

fn solve(grid: &grid::Grid<Memory>) -> Option<(Vec<(usize, usize)>, usize)> {
    let start = (0, 0);
    let end = (grid.width() - 1, grid.height() - 1);
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
}

fn part1(input: &str, size: usize, wait: usize) -> usize {
    let positions = parse(input);
    let mut grid = grid::Grid::blank(size, size, Memory::Safe);
    positions.iter().take(wait).for_each(|pos| {
        grid[pos] = Memory::Corrupt;
    });
    solve(&grid).unwrap().1
}

fn part2(input: &str, size: usize) -> (usize, usize) {
    let positions = parse(input);
    let grid = grid::Grid::blank(size, size, Memory::Safe);
    let last_solvable = (0..positions.len() - 1)
        .rfind(|i| {
            let mut grid = grid.clone();
            positions.iter().take(i + 1).for_each(|pos| {
                grid[pos] = Memory::Corrupt;
            });
            solve(&grid).is_some()
        })
        .unwrap();
    let found = positions[last_solvable + 1];
    (found.1, found.0)
}

fn main() {
    const TEST: &str = include_str!("./input/test.txt");
    const INPUT: &str = include_str!("./input/input.txt");
    test_part1(|| part1(TEST, 7, 12), 22);
    answer_part1(|| part1(INPUT, 71, 1024), 272);
    test_part2(|| part2(TEST, 7), (6, 1));
    answer_part2(|| part2(INPUT, 71), (16, 44));
}
