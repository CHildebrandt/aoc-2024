use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use utils::*;

fn part1(input: &str) -> usize {
    let all_sorted = input
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .unique()
        .collect_vec();
    let mut map = HashMap::<&str, HashSet<&str>>::new();
    all_sorted.iter().for_each(|(key, val)| {
        map.entry(*key).or_default().insert(*val);
        map.entry(*val).or_default().insert(*key);
    });
    map.iter()
        .filter(|(key, _)| key.starts_with("t"))
        .flat_map(|(a, set)| {
            set.iter()
                .flat_map(|b| {
                    map.get(*b)
                        .unwrap()
                        .intersection(set)
                        .map(|c| {
                            let list = &mut [*a, *b, *c];
                            list.sort();
                            (list[0], list[1], list[2])
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .unique()
        .count()
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    part1_test!(7);
    part1_answer!(1411);
    // part2_test!(0);
    // part2_answer!(0);
}
