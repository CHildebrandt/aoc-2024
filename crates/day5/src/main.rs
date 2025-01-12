use itertools::Itertools;
use std::collections::HashSet;
use utils::*;

fn parse(input: &str) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let (ordering_rules, updates) = utils::split_double_newline_once(input);
    (
        ordering_rules
            .lines()
            .map(|line| line.split_once("|").unwrap())
            .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
            .collect::<Vec<_>>(),
        updates
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    )
}

fn part1(input: &str) -> usize {
    let (ordering_rules, updates) = parse(input);
    updates
        .iter()
        .filter(|page_nums| {
            let mut set = HashSet::<usize>::new();
            page_nums.iter().all(|page_num| {
                if set.iter().any(|visited| visited == page_num) {
                    false
                } else {
                    set.extend(
                        ordering_rules
                            .iter()
                            .filter(|rule| rule.1 == *page_num)
                            .map(|rule| rule.0),
                    );
                    true
                }
            })
        })
        .map(|instruction| instruction[instruction.len() / 2])
        .sum()
}

fn part2(input: &str) -> usize {
    let (ordering_rules, mut updates) = parse(input);
    updates
        .iter_mut()
        .filter(|page_nums| {
            let mut set = HashSet::<usize>::new();
            !page_nums.iter().all(|page_num| {
                if set.iter().any(|visited| visited == page_num) {
                    false
                } else {
                    set.extend(
                        ordering_rules
                            .iter()
                            .filter(|rule| rule.1 == *page_num)
                            .map(|rule| rule.0),
                    );
                    true
                }
            })
        })
        .update(|page_nums| {
            page_nums.sort_by(|a, b| {
                if ordering_rules
                    .iter()
                    .any(|rule| rule.1 == *a && rule.0 == *b)
                {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            });
        })
        .map(|instruction| instruction[instruction.len() / 2])
        .sum()
}

fn main() {
    part1_test!(143);
    part1_answer!(6260);
    part2_test!(123);
    part2_answer!(5346);
}
