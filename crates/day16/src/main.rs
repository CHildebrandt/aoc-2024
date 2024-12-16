use direction::CardinalDirection;
use grid::{Grid, Position};
use itertools::Itertools;
use pathfinding::prelude::{astar_bag, AstarSolution};
use utils::*;

fn parse(input: &str) -> (AstarSolution<(Position, CardinalDirection)>, usize) {
    let grid = Grid::char_grid(input);
    let start = grid.find(|c| c == &'S').unwrap();
    let end = grid.find(|c| c == &'E').unwrap();
    astar_bag(
        &(start, CardinalDirection::East),
        |(curr, facing)| {
            grid.neighbors_cardinal(*curr)
                .iter()
                .filter_map(|next| {
                    if grid.get(*next).unwrap() == &'#' {
                        None
                    } else {
                        let diff = (
                            next.0 as isize - curr.0 as isize,
                            next.1 as isize - curr.1 as isize,
                        );
                        let next_facing = CardinalDirection::from_diff(diff).unwrap();
                        if facing.is_180(&next_facing) {
                            None
                        } else {
                            let cost = if facing != &next_facing { 1001 } else { 1 };
                            Some(((*next, next_facing), cost))
                        }
                    }
                })
                .collect_vec()
        },
        |(pos, _)| grid.distance_cardinal(*pos, end),
        |(pos, _)| pos == &end,
    )
    .unwrap()
}

fn part1(input: &str) -> usize {
    parse(input).1
}

fn part2(input: &str) -> usize {
    parse(input)
        .0
        .flat_map(|steps| steps.iter().map(|(pos, _)| *pos).collect_vec())
        .unique()
        .count()
}

fn main() {
    part1_test!(11048);
    part1_answer!(65436);
    part2_test!(64);
    part2_answer!(489);
}
