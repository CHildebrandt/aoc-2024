use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use utils::*;

fn is_possible_design(patterns: &HashSet<&str>, design: &str, start: usize) -> bool {
    for n in start + 1..design.len() {
        if patterns.contains(&design[start..n]) && n == design.len() - 1 {
            return true;
        }
        for pattern in patterns {
            if &design[start..n] == *pattern {
                if is_possible_design(patterns, design, n) {
                    return true;
                }
            }
        }
    }
    false
}

fn count_possible_designs(
    design: &str,
    cursor: usize,
    patterns: &HashSet<&str>,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if cursor == design.len() {
        return 1;
    }
    if let Some(&num_combinations) = memo.get(&cursor) {
        return num_combinations;
    }
    let num_combinations = patterns
        .iter()
        .filter(|pattern| design[cursor..].starts_with(*pattern))
        .fold(0, |acc, pattern| {
            acc + count_possible_designs(design, cursor + pattern.len(), patterns, memo)
        });
    memo.insert(cursor, num_combinations);
    num_combinations
}

fn part1(input: &str) -> usize {
    let (patterns, designs) = split_double_newline_once(input);
    let patterns = patterns.split(", ").collect::<HashSet<_>>();
    let designs = designs.lines().collect_vec();
    designs
        .iter()
        .filter(|design| is_possible_design(&patterns, design, 0))
        .count()
}

fn part2(input: &str) -> usize {
    let (patterns, designs) = split_double_newline_once(input);
    let patterns = patterns.split(", ").collect::<HashSet<_>>();
    let designs = designs.lines().collect_vec();
    designs
        .iter()
        .map(|design| count_possible_designs(design, 0, &patterns, &mut HashMap::new()))
        .sum()
}

fn main() {
    part1_test!(6);
    part1_answer!(216);
    part2_test!(16);
    part2_answer!(603191454138773);
}
