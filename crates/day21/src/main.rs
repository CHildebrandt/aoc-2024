use std::{collections::HashMap, fmt::Display};

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

impl Display for LockKeypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Seven => write!(f, "7"),
            Self::Eight => write!(f, "8"),
            Self::Nine => write!(f, "9"),
            Self::Four => write!(f, "4"),
            Self::Five => write!(f, "5"),
            Self::Six => write!(f, "6"),
            Self::One => write!(f, "1"),
            Self::Two => write!(f, "2"),
            Self::Three => write!(f, "3"),
            Self::Panic => write!(f, "_"),
            Self::Zero => write!(f, "0"),
            Self::A => write!(f, "A"),
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

impl Display for RobotKeypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Panic => write!(f, "_"),
            Self::A => write!(f, "A"),
            Self::Up => write!(f, "^"),
            Self::Left => write!(f, "<"),
            Self::Down => write!(f, "v"),
            Self::Right => write!(f, ">"),
        }
    }
}

impl RobotKeypad {
    fn from_cardinal_direction(dir: CardinalDirection) -> Self {
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
    for (a, b) in path.iter().tuple_windows() {
        let dir = CardinalDirection::from_diff((
            b.0 as isize - a.0 as isize,
            b.1 as isize - a.1 as isize,
        ))
        .unwrap();
        let keypad_press = RobotKeypad::from_cardinal_direction(dir);
        sequence.push(keypad_press);
    }
    sequence.push(RobotKeypad::A);
    sequence
}

fn get_num_presses(
    inputs: Vec<RobotKeypad>,
    depth: usize,
    pad_to_pos: &HashMap<RobotKeypad, Position>,
    robot_keypad_lookup: &HashMap<(RobotKeypad, RobotKeypad), Vec<Vec<RobotKeypad>>>,
    cache: &mut HashMap<(Vec<RobotKeypad>, usize), usize>,
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
        cache.insert((inputs, depth), min);
        min
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
        .collect::<Vec<_>>();

    let lock_keypad = Grid::from_2d_slice(LOCK_KEYPAD);
    let lock_paths = lock_keypad
        .iter_positions()
        .filter(|pos| lock_keypad[*pos] != LockKeypad::Panic)
        .collect::<Vec<_>>()
        .iter()
        .tuple_combinations()
        .fold(HashMap::new(), |mut map, (a, b)| {
            let solution = lock_keypad.astar_bag_cardinal(a, b).unwrap().0;
            let mut forwards = vec![];
            let mut backwards = vec![];
            solution.for_each(|solution| {
                let mut path = solution.clone();
                forwards.push(path.clone());
                path.reverse();
                backwards.push(path);
            });
            map.insert((*a, *b), forwards);
            map.insert((*b, *a), backwards);
            map
        });
    let lock_key_to_pos = lock_keypad
        .iter_positions()
        .filter(|pos| lock_keypad[*pos] != LockKeypad::Panic)
        .map(|pos| (lock_keypad[pos], pos))
        .collect::<HashMap<_, _>>();
    let robot_keypad = Grid::from_2d_slice(ROBOT_KEYPAD);
    let robot_paths = robot_keypad
        .iter_positions()
        .filter(|pos| robot_keypad[*pos] != RobotKeypad::Panic)
        .collect::<Vec<_>>()
        .iter()
        .tuple_combinations()
        .fold(HashMap::new(), |mut map, (a, b)| {
            let solution = robot_keypad.astar_bag_cardinal(a, b).unwrap().0;
            let mut forwards = vec![];
            let mut backwards = vec![];
            solution.for_each(|solution| {
                let mut path = solution.clone();
                forwards.push(path.clone());
                path.reverse();
                backwards.push(path);
            });
            map.insert((*a, *b), forwards);
            map.insert((*b, *a), backwards);
            map
        });
    let lookup = robot_paths
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
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    let mut cache = HashMap::new();
    for (code, code_factor) in codes.iter().zip(code_vals) {
        let presses = code.iter().tuple_windows().fold(0, |acc, (curr, next)| {
            let lock_paths = &lock_paths[&(lock_key_to_pos[&curr], lock_key_to_pos[next])];
            let min = lock_paths
                .iter()
                .map(|path| {
                    let sequence = sequence_from_path(path);
                    get_num_presses(sequence, depth - 1, &pad_to_pos, &lookup, &mut cache)
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
