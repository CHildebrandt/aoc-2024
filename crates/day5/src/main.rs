use std::collections::{HashMap, HashSet};

fn part1(input: &str) -> usize {
    let (ordering_rules, updates) = input.split_once("\r\n\r\n").unwrap();
    let ordering_rules = ordering_rules
        .lines()
        .map(|line| {
            line.split_once("|")
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .unwrap()
        })
        .collect::<Vec<_>>();
    let updates = updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let valid_updates = updates
        .iter()
        .filter(|page_nums| {
            let mut visited = HashSet::new();
            page_nums.iter().all(|page_num| {
                if visited.contains(page_num) {}
                if ordering_rules
                    .iter()
                    .filter(|rule| visited.contains(page_num) && rule.0 == *page_num)
                    .any(|rule| rule.1 > *page_num)
                {
                    println!("Invalid page number: {}", page_num);
                    println!("{:?}", visited);
                    return false;
                }
                visited.extend(
                    ordering_rules
                        .iter()
                        .filter(|rule| rule.0 == *page_num)
                        .map(|rule| rule.0),
                );
                true
            })
        })
        .collect::<Vec<_>>();
    println!("{:?}", valid_updates);
    valid_updates.iter().fold(0, |acc, instruction| {
        acc + instruction[(instruction.len() as f32 / 2.0).floor() as usize]
    })
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    assert_eq!(part1(include_str!("./test")), 143);
    // assert_eq!(part1(include_str!("./input")), 0);
    // assert_eq!(part2(include_str!("./test")), 0);
    // assert_eq!(part2(include_str!("./input")), 0);
}
