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
    let mut lock_grids = vec![];
    let mut key_grids = vec![];
    for grid in split_double_newline(input)
        .iter()
        .map(|grid| Grid::from_str(&grid, Tile::from))
    {
        let is_lock = grid[(0usize, 0usize)] == Tile::Filled;
        let items = grid
            .iter_cols()
            .map(|col| {
                col.iter()
                    .take(if is_lock { col.len() } else { col.len() - 1 })
                    .skip(1)
                    .filter(|&tile| tile == &Tile::Filled)
                    .count()
            })
            .collect_vec();
        if is_lock {
            lock_grids.push(items);
        } else {
            key_grids.push(items);
        }
    }
    lock_grids
        .iter()
        .cartesian_product(key_grids.iter())
        .fold(0, |acc, (lock, key)| {
            if lock
                .iter()
                .zip(key.iter())
                .all(|(lock_height, key_height)| lock_height + key_height <= 5)
            {
                acc + 1
            } else {
                acc
            }
        })
}

fn main() {
    part1_test!(3);
    part1_answer!(2618);
}
