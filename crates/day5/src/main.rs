use itertools::Itertools;
use std::collections::HashSet;

fn parse(input: &str) -> (Vec<(&str, &str)>, Vec<Vec<&str>>) {
    let split_point = input.lines().position(|line| line.is_empty()).unwrap();
    let ordering_rules = input
        .lines()
        .take(split_point)
        .map(|line| line.split_once("|").unwrap())
        .collect::<Vec<_>>();
    let updates = input
        .lines()
        .skip(split_point + 1)
        .map(|line| line.split(",").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (ordering_rules, updates)
}

fn part1(input: &str) -> usize {
    let (ordering_rules, updates) = parse(input);
    let valid_updates = updates
        .iter()
        .filter(|page_nums| {
            let mut set = HashSet::<&str>::new();
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
        .collect::<Vec<_>>();
    valid_updates.iter().fold(0, |acc, instruction| {
        acc + instruction[(instruction.len() as f32 / 2.0).floor() as usize]
            .parse::<usize>()
            .unwrap()
    })
}

fn part2(input: &str) -> usize {
    let (ordering_rules, mut updates) = parse(input);
    updates
        .iter_mut()
        .filter(|page_nums| {
            let mut set = HashSet::<&str>::new();
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
        .fold(0, |acc, instruction| {
            acc + instruction[(instruction.len() as f32 / 2.0).floor() as usize]
                .parse::<usize>()
                .unwrap()
        })
}

fn main() {
    assert_eq!(part1(include_str!("./test")), 143);
    assert_eq!(part1(include_str!("./input")), 6260);
    assert_eq!(part2(include_str!("./test")), 123);
    assert_eq!(part2(include_str!("./input")), 5346);
}
