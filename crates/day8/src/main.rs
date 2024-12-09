use grid::Grid;
use itertools::Itertools;
use utils::*;

const TEST: &str = include_str!("./input/test.txt");
const INPUT: &str = include_str!("./input/input.txt");

fn part1(input: &str) -> usize {
    let grid = Grid::char_grid(input);
    grid.group_by_cell_value()
        .iter()
        .filter_map(|(k, v)| (*k != '.' && v.len() > 1).then_some(v))
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
    let grid = Grid::char_grid(input);
    grid.group_by_cell_value()
        .iter()
        .filter_map(|(k, v)| (*k != '.' && v.len() > 1).then_some(v))
        .flat_map(|antenna_group| {
            antenna_group
                .iter()
                .tuple_combinations()
                .flat_map(|(pos_a, pos_b)| grid.harmonics(*pos_a, *pos_b))
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
