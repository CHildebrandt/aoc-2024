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

fn part1(input: &str) -> usize {
    let mut secret_numbers = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect_vec();
    for _ in 0..2000 {
        secret_numbers.iter_mut().for_each(update_secret);
    }
    secret_numbers.iter().sum()
}

fn part2(input: &str) -> usize {
    // let mut secret_numbers = input
    //     .lines()
    //     .map(|line| line.parse::<usize>().unwrap())
    //     .collect_vec();
    let mut secret_numbers = vec![123];
    let mut changes: Vec<Vec<(usize, Option<isize>)>> = vec![vec![]; secret_numbers.len()];
    for _ in 0..10 {
        for (i, (num, changes)) in secret_numbers
            .iter_mut()
            .zip(changes.iter_mut())
            .enumerate()
        {
            let before = *num;
            let last_digit_before = extract_last_digit(before);
            update_secret(num);
            let last_digit_after = extract_last_digit(before);
            let change = if changes.is_empty() {
                None
            } else {
                Some(last_digit_after as isize - changes.last().unwrap().0 as isize)
            };
            println!("{}: {} ({:?})", before, last_digit_before, change);
            changes.push((last_digit_after, change));
        }
    }
    changes
        .iter()
        .map(|changes| {
            let sequences = changes
                .iter()
                .skip(1)
                .map(|x| (x.0, x.1.unwrap()))
                .take(10)
                .tuple_windows()
                .filter(|(a, b, c, d)| a.1 != b.1 && b.1 != c.1 && c.1 != d.1)
                .collect_vec();
            changes
        })
        .collect_vec();
    0
}

fn main() {
    part1_test!(37327623);
    part1_answer!(18317943467);
    // test_part2(|| part2(include_str!("./input/test2.txt")), 23);
    // part2_test!(0);
    // part2_answer!(0);
}
