use itertools::Itertools;
use utils::*;

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(utils::whitespaced_ints)
        .filter(|row| {
            let ascending = row[0] < row[1];
            !row.iter().tuple_windows().any(|(curr, next)| {
                (ascending && curr >= next)
                    || (!ascending && curr <= next)
                    || curr.abs_diff(*next) > 3
            })
        })
        .count()
}

fn check(row: &Vec<usize>) -> bool {
    let diffs = row
        .iter()
        .tuple_windows()
        .map(|(curr, next)| *next as isize - *curr as isize)
        .collect::<Vec<_>>();
    diffs.iter().all(|diff| (1..=3).contains(diff))
        || diffs.iter().all(|diff| (-3..=-1).contains(diff))
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(utils::whitespaced_ints)
        .filter(|row| {
            check(&row)
                || row.iter().enumerate().any(|(i, _)| {
                    let mut row = row.clone();
                    row.remove(i);
                    check(&row)
                })
        })
        .count()
}

fn main() {
    part1_test!(2);
    part1_answer!(369);
    part2_test!(4);
    part2_answer!(428);
}
