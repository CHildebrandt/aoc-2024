use utils::*;

const TEST: &str = include_str!("./input/test.txt");
const INPUT: &str = include_str!("./input/input.txt");

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(l, r)| {
                    (
                        l.parse().unwrap(),
                        r.split_whitespace().map(|x| x.parse().unwrap()).collect(),
                    )
                })
                .unwrap()
        })
        .collect()
}

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
    Concat,
}

fn can_eval_to(nums: &[usize], value: usize, using_ops: &[Op]) -> bool {
    if nums.len() == 1 {
        nums[0] == value
    } else {
        let (last, rest) = nums.split_last().unwrap();
        using_ops.iter().any(|op| match op {
            Op::Add => value >= *last && can_eval_to(rest, value - last, using_ops),
            Op::Mul => value % last == 0 && can_eval_to(rest, value / last, using_ops),
            Op::Concat => {
                let value_str = value.to_string();
                let last_str = last.to_string();
                value_str.ends_with(&last_str)
                    && can_eval_to(
                        rest,
                        value_str[..value_str.len() - last_str.len()]
                            .parse()
                            .unwrap(),
                        using_ops,
                    )
            }
        })
    }
}

fn part1(input: &str) -> usize {
    parse(input).iter().fold(0, |acc, (sum, nums)| {
        can_eval_to(nums, *sum, &[Op::Add, Op::Mul])
            .then_some(acc + sum)
            .unwrap_or(acc)
    })
}

fn part2(input: &str) -> usize {
    parse(input).iter().fold(0, |acc, (sum, nums)| {
        can_eval_to(nums, *sum, &[Op::Add, Op::Mul, Op::Concat])
            .then_some(acc + sum)
            .unwrap_or(acc)
    })
}

fn main() {
    test_part1(|| part1(TEST), 3749);
    answer_part1(|| part1(INPUT), 975671981569);
    test_part2(|| part2(TEST), 11387);
    answer_part2(|| part2(INPUT), 223472064194845);
}
