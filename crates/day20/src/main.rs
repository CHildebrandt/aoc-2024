use grid::{Grid, Obstructs};
use itertools::Itertools;
use pathfinding::prelude::astar;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

fn solve(input: &str, min_save: usize, jump_distance: usize) -> usize {
    let grid = Grid::char_grid(input);
    let start = grid.find(|c| c == &'S').unwrap();
    let end = grid.find(|c| c == &'E').unwrap();
    let grid = Grid::from_str(input, Tile::from);
    let min_score = grid.astar_cardinal(&start, &end).unwrap().1;
    let empty = grid
        .iter()
        .filter_map(|(pos, &tile)| (tile == Tile::Empty).then_some(pos))
        .collect_vec();
    empty
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| grid.distance_cardinal(**a, **b) <= jump_distance)
        .filter(|(a, b)| {
            grid.astar_cardinal(a, b)
                .and_then(|(_, score)| (score > min_save).then_some(score))
                .is_some()
        })
        .filter(|&(a, b)| {
            astar(
                &(false, start),
                |(has_jumped, pos)| {
                    if !has_jumped && pos == a {
                        vec![((true, *b), grid.distance_cardinal(*pos, *b))]
                    } else if !has_jumped && pos == b {
                        vec![((true, *a), grid.distance_cardinal(*pos, *a))]
                    } else {
                        grid.neighbor_iter_unobstructed_cardinal(pos)
                            .map(|(pos, _)| ((*has_jumped, pos), 1))
                            .collect()
                    }
                },
                |(_, pos)| grid.distance_cardinal(*pos, end),
                |(_, pos)| pos == &end,
            )
            .is_some_and(|(_, score)| score <= min_score - min_save)
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
    // answer_part2(|| part2(INPUT, 100), 1518);
}
