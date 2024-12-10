use grid::Grid;
use pathfinding::prelude::{astar, astar_bag};

use utils::*;

const TEST: &str = include_str!("./input/test.txt");
const INPUT: &str = include_str!("./input/input.txt");

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input, |c| c.to_digit(10).unwrap() as usize);
    let trail_heads = grid.get_positions_where(|v| *v == 0);
    let peaks = grid.get_positions_where(|v| *v == 9);
    trail_heads.iter().fold(0, |sum, trail_head| {
        sum + peaks
            .iter()
            .filter(|peak| {
                astar(
                    trail_head,
                    |curr_pos| {
                        grid.neighbors_cardinal(*curr_pos)
                            .iter()
                            .filter(|neighbor_pos| {
                                let curr_height = grid.get(*curr_pos).unwrap();
                                let next_height = *grid.get(**neighbor_pos).unwrap();
                                next_height == curr_height + 1
                            })
                            .map(|pos| (*pos, 1))
                            .collect::<Vec<_>>()
                    },
                    |pos| grid.distance_cardinal(*pos, **peak),
                    |pos| pos == *peak,
                )
                .is_some()
            })
            .count()
    })
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input, |c| c.to_digit(10).unwrap() as usize);
    let trail_heads = grid.get_positions_where(|v| *v == 0);
    let peaks = grid.get_positions_where(|v| *v == 9);
    trail_heads.iter().fold(0, |sum, trail_head| {
        sum + peaks
            .iter()
            .filter_map(|peak| {
                astar_bag(
                    trail_head,
                    |curr_pos| {
                        grid.neighbors_cardinal(*curr_pos)
                            .iter()
                            .filter(|neighbor_pos| {
                                let curr_height = grid.get(*curr_pos).unwrap();
                                let next_height = *grid.get(**neighbor_pos).unwrap();
                                next_height == curr_height + 1
                            })
                            .map(|pos| (*pos, 1))
                            .collect::<Vec<_>>()
                    },
                    |pos| grid.distance_cardinal(*pos, *peak),
                    |pos| pos == peak,
                )
                .map(|(solution, _)| solution.count())
            })
            .sum::<usize>()
    })
}

fn main() {
    test_part1(|| part1(TEST), 36);
    answer_part1(|| part1(INPUT), 510);
    test_part2(|| part2(TEST), 81);
    answer_part2(|| part2(INPUT), 1058);
}
