fn part1(input: &str) -> usize {
    let list = input
        .lines()
        .into_iter()
        .map(|input| {
            let (l, r) = input.split_at(input.len() / 2);
            (
                l.trim().parse::<usize>().unwrap(),
                r.trim().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut left_sorted = list.iter().map(|(l, _)| l).collect::<Vec<_>>();
    left_sorted.sort();
    let mut right_sorted = list.iter().map(|(_, r)| r).collect::<Vec<_>>();
    right_sorted.sort();
    left_sorted
        .iter()
        .zip(right_sorted.iter())
        .fold(0, |acc, (l, r)| acc + **l.max(r) - **l.min(r))
}

fn part2(input: &str) -> usize {
    let list = input
        .lines()
        .into_iter()
        .map(|input| {
            let (l, r) = input.split_at(input.len() / 2);
            (
                l.trim().parse::<usize>().unwrap(),
                r.trim().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    list.iter().fold(0, |acc, (l, _)| {
        let occurences = list
            .iter()
            .fold(0, |acc, (_, r)| if l == r { acc + 1 } else { acc });
        acc + *l * occurences
    })
}

fn main() {
    assert_eq!(part1(include_str!("./test")), 11);
    assert_eq!(part1(include_str!("./input")), 1222801);
    assert_eq!(part2(include_str!("./test")), 31);
    assert_eq!(part2(include_str!("./input")), 22545250);
}
