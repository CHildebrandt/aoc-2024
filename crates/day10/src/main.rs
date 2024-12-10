use grid::{Grid, Position};
use pathfinding::prelude::{astar, astar_bag};

use utils::*;

const TEST: &str = include_str!("./input/test.txt");
const INPUT: &str = include_str!("./input/input.txt");

struct HeightMap {
    grid: Grid<usize>,
    trail_heads: Vec<Position>,
    peaks: Vec<Position>,
}

impl HeightMap {
    fn from_str(input: &str) -> Self {
        let grid = Grid::from_str(input, |c| c.to_digit(10).unwrap() as usize);
        let trail_heads = grid.get_positions_where(|v| *v == 0);
        let peaks = grid.get_positions_where(|v| *v == 9);
        Self {
            grid,
            trail_heads,
            peaks,
        }
    }

    fn neighbors(&self, pos: &Position) -> Vec<(Position, usize)> {
        self.grid
            .neighbors_cardinal(*pos)
            .iter()
            .filter(|neighbor_pos| {
                let curr_height = self.grid.get(*pos).unwrap();
                let next_height = *self.grid.get(**neighbor_pos).unwrap();
                next_height == curr_height + 1
            })
            .map(|pos| (*pos, 1))
            .collect::<Vec<_>>()
    }

    fn heuristic(&self, pos: Position, peak: Position) -> usize {
        self.grid.distance_cardinal(pos, peak)
    }
}

fn part1(input: &str) -> usize {
    let map = HeightMap::from_str(input);
    map.trail_heads.iter().fold(0, |sum, trail_head| {
        sum + map
            .peaks
            .iter()
            .filter(|peak| {
                astar(
                    trail_head,
                    |curr_pos| map.neighbors(curr_pos),
                    |pos| map.heuristic(*pos, **peak),
                    |pos| pos == *peak,
                )
                .is_some()
            })
            .count()
    })
}

fn part2(input: &str) -> usize {
    let map = HeightMap::from_str(input);
    map.trail_heads.iter().fold(0, |sum, trail_head| {
        sum + map.peaks.iter().fold(0, |sum, peak| {
            sum + astar_bag(
                trail_head,
                |curr_pos| map.neighbors(curr_pos),
                |pos| map.heuristic(*pos, *peak),
                |pos| pos == peak,
            )
            .map(|(solution, _)| solution.count())
            .unwrap_or(0)
        })
    })
}

fn main() {
    test_part1(|| part1(TEST), 36);
    answer_part1(|| part1(INPUT), 510);
    test_part2(|| part2(TEST), 81);
    answer_part2(|| part2(INPUT), 1058);
}
