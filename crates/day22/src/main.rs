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

fn part1(input: &str) -> usize {
    let mut secret_numbers = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec();
    for _ in 0..2000 {
        for num in secret_numbers.iter_mut() {
            *num = mix_and_prune(*num, *num * 64);
            *num = mix_and_prune(*num, (*num as f64 / 32.0) as usize);
            *num = mix_and_prune(*num, *num * 2048);
        }
    }
    secret_numbers.iter().sum()
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    part1_test!(37327623);
    part1_answer!(0);
    // part2_test!(0);
    // part2_answer!(0);
}
