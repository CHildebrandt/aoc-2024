use grid::Grid;
use itertools::Itertools;
use utils::*;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Filled,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Filled,
            _ => panic!("Invalid tile"),
        }
    }
}

fn part1(input: &str) -> usize {
    let grids = split_double_newline(input)
        .iter()
        .map(|grid| Grid::from_str(&grid, Tile::from))
        .collect_vec();
    let mut lock_grids = vec![];
    let mut key_grids = vec![];
    for grid in grids.iter() {
        let is_lock = grid[(0usize, 0usize)] == Tile::Filled;
        if is_lock {
            lock_grids.push(
                grid.iter_cols()
                    .map(|col| {
                        col.iter()
                            .skip(1)
                            .filter(|&tile| tile == &Tile::Filled)
                            .count()
                    })
                    .collect_vec(),
            );
        } else {
            key_grids.push(
                grid.iter_cols()
                    .map(|col| {
                        col.iter()
                            .take(col.len() - 1)
                            .filter(|&tile| tile == &Tile::Filled)
                            .count()
                    })
                    .collect_vec(),
            );
        }
    }
    let mut count = 0;
    for lock_heights in lock_grids.iter() {
        for key_heights in key_grids.iter() {
            if lock_heights
                .iter()
                .zip(key_heights.iter())
                .all(|(lock_height, key_height)| lock_height + key_height <= 5)
            {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    part1_test!(3);
    part1_answer!(2618);
}
