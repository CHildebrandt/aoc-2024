use std::collections::HashMap;

use itertools::Itertools;
use utils::*;

fn mix(secret: usize, next: usize) -> usize {
    secret ^ next
}
fn prune(secret: usize) -> usize {
    secret % 16777216
}
fn mix_and_prune(secret: usize, next: usize) -> usize {
    prune(mix(secret, next))
}
fn update_secret(secret: &mut usize) {
    *secret = mix_and_prune(*secret, *secret * 64);
    *secret = mix_and_prune(*secret, (*secret as f64 / 32.0) as usize);
    *secret = mix_and_prune(*secret, *secret * 2048);
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(input: &str) -> usize {
    let mut secret_numbers = parse_input(input);
    for _ in 0..2000 {
        secret_numbers.iter_mut().for_each(update_secret);
    }
    secret_numbers.iter().sum()
}

fn part2(input: &str) -> usize {
    let mut secret_numbers = parse_input(input);
    let mut price_changes: Vec<Vec<(usize, Option<isize>)>> = vec![vec![]; secret_numbers.len()];
    for _ in 0..2000 {
        for (num, changes) in secret_numbers.iter_mut().zip(price_changes.iter_mut()) {
            let last_digit_before = extract_last_digit(*num);
            update_secret(num);
            let change = if changes.is_empty() {
                None
            } else {
                Some(last_digit_before as isize - changes.last().unwrap().0 as isize)
            };
            changes.push((last_digit_before, change));
        }
    }
    let mut sequence_map = HashMap::new();
    price_changes.iter().for_each(|changes| {
        changes
            .iter()
            .skip(1)
            .map(|x| (x.0, x.1.unwrap()))
            .tuple_windows()
            .filter(|(a, b, c, d)| a.1 != b.1 && b.1 != c.1 && c.1 != d.1)
            .map(|(a, b, c, d)| ((a.1, b.1, c.1, d.1), d.0))
            .unique_by(|(sequence, _)| *sequence)
            .for_each(|(sequence, price_at_sequence)| {
                *sequence_map.entry(sequence).or_insert(0) += price_at_sequence;
            });
    });
    sequence_map
        .iter()
        .map(|(_, bananas)| *bananas)
        .max()
        .unwrap()
}

fn main() {
    part1_test!(37327623);
    part1_answer!(18317943467);
    test_part2(|| part2(include_str!("./input/test2.txt")), 23);
    part2_answer!(2018);
}
