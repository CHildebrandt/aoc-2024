fn part1(input: &str) -> usize {
    let input = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    input.iter().fold(0, |acc, row| {
        let mut iter = row.iter();
        let ascending = iter.next().unwrap() < iter.next().unwrap();
        let mut iter = row.iter().peekable();
        while let Some((curr, next)) = iter.next().zip(iter.peek()) {
            if (ascending && curr >= next)
                || (!ascending && curr <= next)
                || curr.abs_diff(**next) > 3
            {
                return acc;
            }
        }
        acc + 1
    })
}

fn check(row: &Vec<usize>) -> bool {
    let mut diffs: Vec<isize> = vec![];
    let mut iter = row.iter().peekable();
    while let Some((curr, next)) = iter.next().zip(iter.peek()) {
        diffs.push(*curr as isize - **next as isize);
    }
    diffs.iter().all(|diff| (1..=3).contains(diff))
        || diffs.iter().all(|diff| (-3..=-1).contains(diff))
}

fn part2(input: &str) -> usize {
    let input = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    input.iter().fold(0, |acc, row| {
        if check(row) {
            acc + 1
        } else {
            for i in 0..row.len() {
                let mut row = row.clone();
                row.remove(i);
                if check(&row) {
                    return acc + 1;
                }
            }
            acc
        }
    })
}

fn main() {
    assert_eq!(part1(include_str!("./test")), 2);
    assert_eq!(part1(include_str!("./input")), 369);
    assert_eq!(part2(include_str!("./test")), 4);
    assert_eq!(part2(include_str!("./input")), 428);
}
