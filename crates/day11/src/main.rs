use std::collections::HashMap;

use utils::*;

const TEST: &str = include_str!("./input/test.txt");
const INPUT: &str = include_str!("./input/input.txt");

fn calc_recursively(
    stone_val: usize,
    repeat: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if repeat == 0 {
        return 1;
    }
    if let Some(&val) = cache.get(&(stone_val, repeat)) {
        return val;
    }
    let res = if stone_val == 0 {
        calc_recursively(1, repeat - 1, cache)
    } else if (stone_val.ilog10() + 1) % 2 == 0 {
        let s = stone_val.to_string();
        let (a, b) = s.split_at(s.len() / 2);
        calc_recursively(a.parse().unwrap(), repeat - 1, cache)
            + calc_recursively(b.parse().unwrap(), repeat - 1, cache)
    } else {
        calc_recursively(stone_val * 2024, repeat - 1, cache)
    };
    cache.insert((stone_val, repeat), res);
    res
}

fn part1(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .split_whitespace()
        .map(|x| calc_recursively(x.parse().unwrap(), 25, &mut cache))
        .sum()
}

fn part2(input: &str) -> usize {
    let mut cache = HashMap::new();
    input
        .split_whitespace()
        .map(|x| calc_recursively(x.parse().unwrap(), 75, &mut cache))
        .sum()
}

fn main() {
    test_part1(|| part1(TEST), 55312);
    answer_part1(|| part1(INPUT), 199946);
    answer_part2(|| part2(INPUT), 237994815702032);
}
