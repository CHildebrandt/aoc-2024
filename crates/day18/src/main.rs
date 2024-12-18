use grid::{Grid, GridPos, Obstructs};
use itertools::Itertools;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Memory {
    Safe,
    Corrupt,
}

impl Obstructs for Memory {
    fn obstructs(&self) -> bool {
        self == &Memory::Corrupt
    }
}

fn parse(input: &str) -> Vec<GridPos<usize>> {
    input
        .lines()
        .map(|line| GridPos::<usize>::from_str(line).unwrap().flip())
        .collect_vec()
}

fn solve(grid: &Grid<Memory>) -> Option<(Vec<(usize, usize)>, usize)> {
    grid.astar_cardinal(&(0, 0), &(grid.height() - 1, grid.width() - 1))
}

fn part1(input: &str, size: usize, wait: usize) -> usize {
    let positions = parse(input);
    let mut grid = Grid::blank(size, size, Memory::Safe);
    positions.iter().take(wait).for_each(|pos| {
        grid[pos] = Memory::Corrupt;
    });
    solve(&grid).unwrap().1
}

fn part2(input: &str, size: usize) -> (usize, usize) {
    let positions = parse(input);
    let grid = Grid::blank(size, size, Memory::Safe);
    let last_solvable = (0..positions.len() - 1)
        .rfind(|i| {
            let mut grid = grid.clone();
            positions.iter().take(i + 1).for_each(|pos| {
                grid[pos] = Memory::Corrupt;
            });
            solve(&grid).is_some()
        })
        .unwrap();
    positions[last_solvable + 1].flip().into()
}

fn main() {
    const TEST: &str = include_str!("./input/test.txt");
    const INPUT: &str = include_str!("./input/input.txt");
    test_part1(|| part1(TEST, 7, 12), 22);
    answer_part1(|| part1(INPUT, 71, 1024), 272);
    test_part2(|| part2(TEST, 7), (6, 1));
    answer_part2(|| part2(INPUT, 71), (16, 44));
}
