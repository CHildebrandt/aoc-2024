use std::collections::HashSet;

use grid::Grid;
use utils::*;

fn part1(input: &str) -> usize {
    let grid = Grid::char_grid(input);
    let areas = grid.areas(|a, b| a == b);
    let max_y = grid.height() - 1;
    let max_x = grid.width() - 1;
    println!("{:?}", areas);
    areas.iter().fold(0, |acc, area| {
        let mut perimeter = 0;
        let area_set = area.iter().map(|(pos, _)| *pos).collect::<HashSet<_>>();
        for (pos, _) in area {
            match pos {
                (0, 0) => {
                    perimeter += 2;
                }
                (0, _) => {
                    if pos.1 == max_x || pos.1 == 0 {
                        perimeter += 2;
                    } else {
                        perimeter += 1;
                    }
                }
                (_, 0) => {
                    perimeter += 1;
                }
                _ => {
                    if pos.0 == max_y && pos.1 == max_x {
                        perimeter += 2;
                    } else if pos.0 == max_y || pos.1 == max_x {
                        perimeter += 1;
                    }
                }
            }
            let neighbors = grid.neighbors_cardinal(*pos);
            for neighbor in neighbors {
                if !area_set.contains(&neighbor) {
                    perimeter += 1;
                }
            }
            // if pos.0 == 0 || pos.0 == grid.height() - 1 || pos.1 == 0 || pos.1 == grid.width() - 1 {
            //     perimeter += 1;
            // } else {
            //     let neighbors = grid.neighbors_cardinal(*pos);
            //     for neighbor in neighbors {
            //         if !area_set.contains(&neighbor) {
            //             perimeter += 1;
            //         }
            //     }
            // }
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
