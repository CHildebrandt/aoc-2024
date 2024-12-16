use direction::CardinalDirection;
use grid::Grid;
use itertools::Itertools;
use pathfinding::prelude::astar;
use utils::*;

fn part1(input: &str) -> usize {
    let grid = Grid::char_grid(input);
    let start = grid.find(|c| c == &'S').unwrap();
    let end = grid.find(|c| c == &'E').unwrap();
    astar(
        &(start, CardinalDirection::East),
        |(curr, facing)| {
            grid.neighbors_cardinal(*curr)
                .iter()
                .filter_map(|next| {
                    if grid.get(*next).unwrap() == &'#' {
                        None
                    } else {
                        let mut cost = 1;
                        let diff = (
                            next.0 as isize - curr.0 as isize,
                            next.1 as isize - curr.1 as isize,
                        );
                        let next_facing = CardinalDirection::from_diff(diff).unwrap();
                        if facing.is_180(&next_facing) {
                            None
                        } else {
                            if facing != &next_facing {
                                cost += 1000;
                            }
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
    .1
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    part1_test!(11048);
    part1_answer!(65436);
    // part2_test!(0);
    // part2_answer!(0);
}
