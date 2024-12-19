use std::collections::{HashMap, HashSet};
use utils::*;

fn is_possible_design(design: &str, cursor: usize, patterns: &HashSet<&str>) -> bool {
    for n in cursor + 1..design.len() {
        if patterns.contains(&design[cursor..n]) && n == design.len() - 1 {
            return true;
        }
        for pattern in patterns {
            if &design[cursor..n] == *pattern {
                if is_possible_design(design, n, patterns) {
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
        1
    } else if let Some(&num_combinations) = memo.get(&cursor) {
        num_combinations
    } else {
        let num_combinations = patterns
            .iter()
            .filter(|pattern| design[cursor..].starts_with(*pattern))
            .fold(0, |acc, pattern| {
                acc + count_possible_designs(design, cursor + pattern.len(), patterns, memo)
            });
        memo.insert(cursor, num_combinations);
        num_combinations
    }
}

fn parse(input: &str) -> (HashSet<&str>, Vec<&str>) {
    let (patterns, designs) = split_double_newline_once(input);
    (patterns.split(", ").collect(), designs.lines().collect())
}

fn part1(input: &str) -> usize {
    let (patterns, designs) = parse(input);
    designs
        .iter()
        .filter(|design| is_possible_design(design, 0, &patterns))
        .count()
}

fn part2(input: &str) -> usize {
    let (patterns, designs) = parse(input);
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
