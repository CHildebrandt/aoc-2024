use itertools::Itertools;
use utils::*;

const TEST: &str = include_str!("./input/test.txt");
const INPUT: &str = include_str!("./input/input.txt");

#[derive(Debug, Clone)]
struct DiskMapEntry {
    pub id: usize,
    pub used: usize,
    pub free: usize,
}

fn part1(input: &str) -> usize {
    let nums = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    assert!(nums.len() % 2 == 1);
    let mut items = vec![];
    for (curr, next) in nums.iter().tuples() {
        items.push(*curr);
        items.push(*next);
    }
    items.push(*nums.last().unwrap());
    items.push(0); // To make len even
    let with_ids = items
        .iter()
        .tuples()
        .enumerate()
        .map(|(id, (used, free))| DiskMapEntry {
            id,
            used: *used,
            free: *free,
        })
        .collect::<Vec<_>>();
    let mut backwards = with_ids.clone().into_iter().rev().collect::<Vec<_>>();
    let mut nums = vec![];
    let len = backwards.len();
    let mut back_iter = backwards.iter_mut().enumerate();
    let mut backw_curr = back_iter.next().unwrap();
    let mut num_replaced = 0;
    'outer: for (i, forw_curr) in with_ids.iter().enumerate() {
        let j = len - backw_curr.0;
        if i == j {
            break;
        }
        nums.extend(vec![forw_curr.id; forw_curr.used]);
        let mut remainder = forw_curr.free;
        if remainder != 0 {
            loop {
                let mut should_continue = true;
                let count = if backw_curr.1.used >= remainder {
                    should_continue = false;
                    remainder
                } else {
                    remainder -= backw_curr.1.used;
                    backw_curr.1.used
                };
                nums.extend(vec![backw_curr.1.id; count]);
                backw_curr.1.used -= count;
                num_replaced += count;
                if backw_curr.1.used == 0 {
                    if let Some(next) = back_iter.next() {
                        backw_curr = next;
                    } else {
                        break 'outer;
                    }
                }
                if !should_continue {
                    break;
                }
            }
        }
    }
    nums[..nums.len() - num_replaced]
        .iter()
        .enumerate()
        .fold(0, |acc, (i, val)| acc + i * val)
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    test_part1(|| part1(TEST), 1928);
    answer_part1(|| part1(INPUT), 6349606724455);
    // test_part2(|| part2(TEST), 0);
    // answer_part2(|| part2(INPUT), 0);
}
