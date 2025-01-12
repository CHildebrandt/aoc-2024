use itertools::Itertools;
use utils::*;

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let list = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_at(line.len() / 2);
            (
                l.trim().parse::<usize>().unwrap(),
                r.trim().parse::<usize>().unwrap(),
            )
        })
        .collect_vec();
    (
        list.iter().map(|(l, _)| *l).sorted().collect(),
        list.iter().map(|(_, r)| *r).sorted().collect(),
    )
}

fn part1(input: &str) -> usize {
    let (left, right) = parse(input);
    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

fn part2(input: &str) -> usize {
    let (left, right) = parse(input);
    left.iter()
        .map(|l| *l * right.iter().filter(|r| l == *r).count())
        .sum()
}

fn main() {
    part1_test!(11);
    part1_answer!(1222801);
    part2_test!(31);
    part2_answer!(22545250);
}
