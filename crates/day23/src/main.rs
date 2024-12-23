use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use std::iter::once;
use utils::*;

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut map = HashMap::<&str, HashSet<&str>>::new();
    input
        .lines()
        .map(|line| line.split_once("-").unwrap())
        .unique()
        .for_each(|(key, val)| {
            map.entry(key).or_default().insert(val);
            map.entry(val).or_default().insert(key);
        });
    map
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    map.iter()
        .filter(|(key, _)| key.starts_with("t"))
        .flat_map(|(a, set)| {
            set.iter()
                .flat_map(|b| {
                    map.get(b)
                        .unwrap()
                        .intersection(set)
                        .map(|c| {
                            let list = &mut [*a, *b, *c];
                            list.sort();
                            (list[0], list[1], list[2])
                        })
                        .collect_vec()
                })
                .collect_vec()
        })
        .unique()
        .count()
}

fn part2(input: &str) -> String {
    parse(input)
        .iter()
        // Unfortunately, sorting is needed. Might not work with other inputs...
        .sorted_by_key(|(key, _)| *key)
        .fold(vec![], |mut sets: Vec<HashSet<&str>>, (key, set)| {
            if let Some(set) = sets.iter_mut().find(|curr| curr.is_subset(set)) {
                set.insert(*key);
            } else {
                sets.push(HashSet::from_iter(once(*key)));
            }
            sets
        })
        .iter()
        .map(|set| set.iter().sorted().collect_vec())
        .max_by_key(Vec::len)
        .unwrap()
        .iter()
        .sorted()
        .join(",")
}

fn main() {
    part1_test!(7);
    part1_answer!(1411);
    part2_test!("co,de,ka,ta".into());
    part2_answer!("aq,bn,ch,dt,gu,ow,pk,qy,tv,us,yx,zg,zu".into());
}
