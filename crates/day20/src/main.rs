use grid::{Grid, Obstructs};
use itertools::Itertools;
use utils::*;

#[derive(Debug, Clone)]
enum Tile {
    Empty,
    Wall,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            _ => Tile::Empty,
        }
    }
}

impl Obstructs for Tile {
    fn obstructs(&self) -> bool {
        match self {
            Tile::Wall => true,
            _ => false,
        }
    }
}

fn solve(input: &str, min_save: usize, max_cheat: usize) -> usize {
    let grid = Grid::char_grid(input);
    let start = grid.find(|c| c == &'S').unwrap();
    let end = grid.find(|c| c == &'E').unwrap();
    let grid = Grid::from_str(input, Tile::from);
    let win_path = grid.astar_cardinal(&start, &end).unwrap().0;
    win_path
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter(|&((a_steps, a), (b_steps, b))| {
            let a_to_b_cheat_distance = grid.distance_cardinal(a, b);
            a_to_b_cheat_distance <= max_cheat
                && a_steps
                    .abs_diff(b_steps)
                    .checked_sub(a_to_b_cheat_distance)
                    .is_some_and(|saved| saved >= min_save)
        })
        .count()
}

fn part1(input: &str, min_save: usize) -> usize {
    solve(input, min_save, 2)
}

fn part2(input: &str, min_save: usize) -> usize {
    solve(input, min_save, 20)
}

fn main() {
    const TEST: &str = include_str!("./input/test.txt");
    const INPUT: &str = include_str!("./input/input.txt");
    test_part1(|| part1(TEST, 1), 44);
    answer_part1(|| part1(INPUT, 100), 1518);
    test_part2(|| part2(TEST, 50), 285);
    answer_part2(|| part2(INPUT, 100), 1032257);
}
