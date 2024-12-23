use std::collections::HashMap;

use direction::CardinalDirection;
use grid::{Grid, Obstructs, Position};
use itertools::Itertools;
use utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LockKeypad {
    Seven,
    Eight,
    Nine,
    Four,
    Five,
    Six,
    One,
    Two,
    Three,
    Panic,
    Zero,
    A,
}

impl From<char> for LockKeypad {
    fn from(value: char) -> Self {
        match value {
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '1' => Self::One,
            '2' => Self::Two,
            '3' => Self::Three,
            '0' => Self::Zero,
            'A' => Self::A,
            _ => Self::Panic,
        }
    }
}

impl Obstructs for LockKeypad {
    fn obstructs(&self) -> bool {
        self == &Self::Panic
    }
}

const LOCK_KEYPAD: &[&[LockKeypad]] = &[
    &[LockKeypad::Seven, LockKeypad::Eight, LockKeypad::Nine],
    &[LockKeypad::Four, LockKeypad::Five, LockKeypad::Six],
    &[LockKeypad::One, LockKeypad::Two, LockKeypad::Three],
    &[LockKeypad::Panic, LockKeypad::Zero, LockKeypad::A],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RobotKeypad {
    Panic,
    Up,
    A,
    Left,
    Down,
    Right,
}

impl From<CardinalDirection> for RobotKeypad {
    fn from(dir: CardinalDirection) -> Self {
        match dir {
            CardinalDirection::North => Self::Up,
            CardinalDirection::East => Self::Right,
            CardinalDirection::South => Self::Down,
            CardinalDirection::West => Self::Left,
        }
    }
}

impl Obstructs for RobotKeypad {
    fn obstructs(&self) -> bool {
        self == &Self::Panic
    }
}

const ROBOT_KEYPAD: &[&[RobotKeypad]] = &[
    &[RobotKeypad::Panic, RobotKeypad::Up, RobotKeypad::A],
    &[RobotKeypad::Left, RobotKeypad::Down, RobotKeypad::Right],
];

fn sequence_from_path(path: &Vec<Position>) -> Vec<RobotKeypad> {
    let mut sequence = vec![RobotKeypad::A];
    if path.len() <= 1 {
        return sequence;
    }
    for ((ay, ax), (by, bx)) in path.iter().tuple_windows() {
        let direction = CardinalDirection::from_cmp(by.cmp(&ay), bx.cmp(&ax)).unwrap();
        sequence.push(direction.into());
    }
    sequence.push(RobotKeypad::A);
    sequence
}

fn get_num_presses(
    inputs: Vec<RobotKeypad>,
    depth: usize,
    pad_to_pos: &HashMap<RobotKeypad, Position>,
    robot_keypad_lookup: &HashMap<(RobotKeypad, RobotKeypad), Vec<Vec<RobotKeypad>>>,
    cache: &mut cache::Cache<(Vec<RobotKeypad>, usize), usize>,
) -> usize {
    if let Some(cost) = cache.get(&(inputs.clone(), depth)) {
        *cost
    } else {
        let min = inputs
            .iter()
            .tuple_windows()
            .map(|(a, b)| {
                robot_keypad_lookup
                    .get(&(*a, *b))
                    .map(|paths| match depth {
                        0 => paths.first().map(|path| path.len()).unwrap_or(1),
                        _ => paths
                            .iter()
                            .map(|path| {
                                let path = path.iter().map(|key| pad_to_pos[key]).collect_vec();
                                get_num_presses(
                                    sequence_from_path(&path),
                                    depth - 1,
                                    pad_to_pos,
                                    robot_keypad_lookup,
                                    cache,
                                )
                            })
                            .min()
                            .unwrap(),
                    })
                    .unwrap_or(1)
            })
            .sum::<usize>();
        cache.update((inputs, depth), min)
    }
}

fn solve(input: &str, depth: usize) -> usize {
    let code_vals = input
        .lines()
        .map(|line| {
            line.trim_end_matches(char::is_alphabetic)
                .parse::<usize>()
                .unwrap()
        })
        .collect_vec();

    let lock_keypad = Grid::from_2d_slice(LOCK_KEYPAD);
    let lock_paths = lock_keypad.paths_map::<CardinalDirection>();
    let lock_key_to_pos = lock_keypad
        .iter_positions()
        .filter(|pos| lock_keypad[*pos] != LockKeypad::Panic)
        .map(|pos| (lock_keypad[pos], pos))
        .collect::<HashMap<_, _>>();
    let robot_keypad = Grid::from_2d_slice(ROBOT_KEYPAD);
    let robot_paths = robot_keypad.paths_map::<CardinalDirection>();
    let robot_keypad_lookup = robot_paths
        .iter()
        .fold(HashMap::new(), |mut acc, (key, path)| {
            acc.insert(
                (robot_keypad[key.0], robot_keypad[key.1]),
                path.iter()
                    .map(|steps| steps.iter().map(|pos| robot_keypad[*pos]).collect())
                    .collect(),
            );
            acc
        });
    let pad_to_pos = robot_keypad
        .iter_positions()
        .filter(|pos| robot_keypad[*pos] != RobotKeypad::Panic)
        .fold(HashMap::new(), |mut acc, pos| {
            acc.insert(robot_keypad[pos], pos);
            acc
        });
    let codes = input
        .lines()
        .map(|line| {
            format!("A{}", line)
                .chars()
                .map(LockKeypad::from)
                .collect_vec()
        })
        .collect_vec();
    let mut sum = 0;
    let mut cache = Default::default();
    for (code, code_factor) in codes.iter().zip(code_vals) {
        let presses = code.iter().tuple_windows().fold(0, |acc, (curr, next)| {
            let lock_paths = &lock_paths[&(lock_key_to_pos[&curr], lock_key_to_pos[next])];
            let min = lock_paths
                .iter()
                .map(|path| {
                    let sequence = sequence_from_path(path);
                    get_num_presses(
                        sequence,
                        depth - 1,
                        &pad_to_pos,
                        &robot_keypad_lookup,
                        &mut cache,
                    )
                })
                .min()
                .unwrap();
            acc + min
        });
        sum += code_factor * presses;
    }
    sum
}

fn part1(input: &str) -> usize {
    solve(input, 2)
}

fn part2(input: &str) -> usize {
    solve(input, 25)
}

fn main() {
    part1_test!(126384);
    part1_answer!(169390);
    part2_answer!(210686850124870);
}
