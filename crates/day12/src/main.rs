use std::collections::HashSet;

use direction::{CardinalDirection, Direction, PositionVirtual};
use grid::Grid;
use utils::*;

fn part1(input: &str) -> usize {
    let grid = Grid::char_grid(input);
    let areas = grid.areas(|a, b| a == b);
    areas
        .iter()
        .map(|area| {
            let mut perimeter = 0;
            let area_set = area.iter().map(|(pos, _)| *pos).collect::<HashSet<_>>();
            for (pos, _) in area {
                if grid.is_corner(*pos) {
                    perimeter += 2;
                } else if grid.is_edge(*pos) {
                    perimeter += 1;
                }
                let neighbors = grid.neighbors_cardinal(*pos);
                for neighbor in neighbors {
                    if !area_set.contains(&neighbor) {
                        perimeter += 1;
                    }
                }
            }
            area.len() * perimeter
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let grid = Grid::char_grid(input);
    let areas = grid.areas(|a, b| a == b);
    areas
        .iter()
        .map(|area| {
            let mut fence_positions = HashSet::<(&CardinalDirection, PositionVirtual)>::new();
            let area_set = area.iter().map(|(pos, _)| *pos).collect::<HashSet<_>>();
            for (pos, _) in area {
                for direction in CardinalDirection::all() {
                    let diffed = direction.add_unsigned(pos, 1);
                    if !grid.validate_position_virtual(diffed)
                        || !area_set.contains(&(diffed.0 as usize, diffed.1 as usize))
                    {
                        fence_positions.insert((direction, diffed));
                    }
                }
            }
            let mut num_continuous_fence_positions = fence_positions.len();
            for (direction, pos) in fence_positions.iter() {
                for (other_direction, other_pos) in fence_positions.iter() {
                    if direction == other_direction {
                        match direction {
                            CardinalDirection::North | CardinalDirection::South => {
                                if pos.0 == other_pos.0 && pos.1 + 1 == other_pos.1 {
                                    num_continuous_fence_positions -= 1;
                                }
                            }
                            CardinalDirection::East | CardinalDirection::West => {
                                if pos.1 == other_pos.1 && pos.0 + 1 == other_pos.0 {
                                    num_continuous_fence_positions -= 1;
                                }
                            }
                        }
                    }
                }
            }
            area.len() * num_continuous_fence_positions
        })
        .sum()
}

fn main() {
    part1_test!(1930);
    part1_answer!(1361494);
    part2_test!(1206);
    part2_answer!(830516);
}
