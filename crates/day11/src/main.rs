use rayon::vec;
use utils::*;

const TEST: &str = include_str!("./input/test.txt");
const INPUT: &str = include_str!("./input/input.txt");

fn part1(input: &str) -> usize {
    // let mut input = input
    //     .split_whitespace()
    //     .map(|x| vec![x.parse::<usize>().unwrap()])
    //     .collect::<Vec<_>>();
    // for _ in 0..25 {
    //     for stones in input.iter_mut() {
    //         let mut new_stones = vec![];
    //         for stone in stones.iter() {
    //             let (a, b) = if *stone == 0 {
    //                 (1, None)
    //             } else if (stone.ilog10() + 1) % 2 == 0 {
    //                 let s = stone.to_string();
    //                 let (a, b) = s.split_at(s.len() / 2);
    //                 (
    //                     a.parse::<usize>().unwrap(),
    //                     Some(b.parse::<usize>().unwrap()),
    //                 )
    //             } else {
    //                 (stone * 2024, None)
    //             };
    //             new_stones.push(a);
    //             if let Some(b) = b {
    //                 new_stones.push(b);
    //             }
    //         }
    //         *stones = new_stones;
    //     }
    // }
    // input.iter().flatten().collect::<Vec<_>>().len()
    let mut input = input
        .split_whitespace()
        .map(StoneList::from_str)
        .collect::<Vec<_>>();
    input.iter_mut().fold(0, |acc, x| acc + x.blink_n(25))
}

struct StoneList {
    inner: Vec<usize>,
}

impl StoneList {
    fn from_str(input: &str) -> Self {
        Self {
            inner: vec![input.parse::<usize>().unwrap()],
        }
    }

    fn blink_n(&mut self, n: usize) -> usize {
        let mut len = self.inner.len();
        for _ in 0..n {
            for _ in 0..len {
                let curr = self.inner.remove(0);
                if curr == 0 {
                    self.inner.push(1);
                } else if (curr.ilog10() + 1) % 2 == 0 {
                    let s = curr.to_string();
                    let (a, b) = s.split_at(s.len() / 2);
                    self.inner.push(a.parse::<usize>().unwrap());
                    self.inner.push(b.parse::<usize>().unwrap());
                    len += 1;
                } else {
                    self.inner.push(curr * 2024);
                };
            }
        }
        len
    }
}

fn part2(input: &str) -> usize {
    let mut input = input
        .split_whitespace()
        .map(|x| vec![x.parse::<usize>().unwrap()])
        .collect::<Vec<_>>();
    for _ in 0..75 {
        for stones in input.iter_mut() {
            let mut new_stones = vec![];
            for stone in stones.iter() {
                let (a, b) = if *stone == 0 {
                    (1, None)
                } else if (stone.ilog10() + 1) % 2 == 0 {
                    let s = stone.to_string();
                    let (a, b) = s.split_at(s.len() / 2);
                    (
                        a.parse::<usize>().unwrap(),
                        Some(b.parse::<usize>().unwrap()),
                    )
                } else {
                    (stone * 2024, None)
                };
                new_stones.push(a);
                if let Some(b) = b {
                    new_stones.push(b);
                }
            }
            *stones = new_stones;
        }
    }
    input.iter().flatten().collect::<Vec<_>>().len()
}

fn main() {
    test_part1(|| part1(TEST), 55312);
    answer_part1(|| part1(INPUT), 199946);
    // test_part2(|| part2(TEST), 55312);
    // answer_part2(|| part2(INPUT), 0);
}
