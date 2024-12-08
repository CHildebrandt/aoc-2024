use grid::Grid;
use itertools::Itertools;
use utils::*;

const TEST: &str = include_str!("./input/test.txt");
const INPUT: &str = include_str!("./input/input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GridCell {
    Empty,
    Antenna(char),
}

impl GridCell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            c => Self::Antenna(c),
        }
    }
}

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input, GridCell::from_char);
    grid.group_by_cell_value()
        .iter()
        .filter_map(|(k, v)| (k != &GridCell::Empty && v.len() > 1).then_some(v))
        .flat_map(|antenna_group| {
            grid.filter_positions_virtual(
                &antenna_group
                    .iter()
                    .combinations(2)
                    .flat_map(|c| {
                        let (a, b) = grid.get_outer_diff_positions(*c[0], *c[1]);
                        vec![a, b]
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .unique()
        .count()
}

fn part2(input: &str) -> usize {
    let grid = Grid::from_str(input, GridCell::from_char);
    grid.group_by_cell_value()
        .iter()
        .filter_map(|(k, v)| (k != &GridCell::Empty && v.len() > 1).then_some(v))
        .flat_map(|antenna_group| {
            antenna_group
                .iter()
                .combinations(2)
                .flat_map(|c| grid.harmonics(*c[0], *c[1]))
                .collect::<Vec<_>>()
        })
        .unique()
        .count()
}

fn main() {
    test_part1(|| part1(TEST), 14);
    answer_part1(|| part1(INPUT), 367);
    test_part2(|| part2(TEST), 34);
    answer_part2(|| part2(INPUT), 1285);
}
