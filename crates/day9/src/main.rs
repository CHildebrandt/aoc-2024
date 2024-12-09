use itertools::Itertools;
use utils::*;

const TEST: &str = include_str!("./input/test.txt");
const INPUT: &str = include_str!("./input/input.txt");

#[derive(Debug)]
enum DiskItem {
    Free(usize),
    Used(usize),
}

#[derive(Debug, Clone)]
struct DiskMapEntry {
    pub id: usize,
    pub used: usize,
    pub free: usize,
}

impl DiskMapEntry {
    fn has_space(&self) -> bool {
        self.free > 0
    }

    fn has_data(&self) -> bool {
        self.used > 0
    }

    fn move_used(&mut self, other: &mut DiskMapEntry) {
        let rest = if other.free < self.used {
            self.used - other.free
        } else {
            0
        };
        other.free -= self.used;
        other.used += self.used;
        self.used = 0;
    }
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
    let backwards = with_ids.clone().into_iter().rev().collect::<Vec<_>>();
    let mut nums = vec![];
    let mut back_iter = backwards.iter();
    let mut curr = back_iter.next().unwrap().clone();
    for forwards_curr in with_ids.iter() {
        nums.extend(vec![forwards_curr.id; forwards_curr.used]);
        if curr.used > forwards_curr.free {}
        if !curr.has_space() {
            if let Some(next) = back_iter.next() {
                curr = next.clone();
            } else {
                break;
            }
        }
    }
    nums.iter()
        .enumerate()
        .fold(0, |acc, (i, val)| acc + i * val)
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    test_part1(|| part1(TEST), 1928);
    // answer_part1(|| part1(INPUT), 0);
    // test_part2(|| part2(TEST), 0);
    // answer_part2(|| part2(INPUT), 0);
}
