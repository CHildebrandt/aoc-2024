use std::collections::HashSet;

use grid::Grid;
use utils::*;

fn part1(input: &str) -> usize {
    let grid = Grid::char_grid(input);
    let areas = grid.areas(|a, b| a == b);
    println!("{:?}", areas);
    areas.iter().fold(0, |acc, area| {
        let mut perimeter = 0;
        let area_set = area.iter().map(|(pos, _)| *pos).collect::<HashSet<_>>();
        for (pos, _) in area {
            let neighbors = grid.neighbors_cardinal(*pos);
            for neighbor in neighbors {
                if !area_set.contains(&neighbor) {
                    perimeter += 1;
                }
            }
        }
        println!("Len: {:?}", area.len());
        println!("Perimeter: {:?}", perimeter);
        acc + area.len() * perimeter
    })
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    part1_test!(1930);
    // part1_answer!(0);
    // part2_test!(0);
    // part2_answer!(0);
}
