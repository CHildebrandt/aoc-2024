use direction::{CardinalDirection, Direction};
use itertools::Itertools;
use utils::*;

fn dir_from_char(c: char) -> CardinalDirection {
    match c {
        '<' => CardinalDirection::West,
        '^' => CardinalDirection::North,
        '>' => CardinalDirection::East,
        'v' => CardinalDirection::South,
        _ => panic!("Invalid direction {}", c),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Empty,
    Wall,
    Box,
    Robot,
    BoxL,
    BoxR,
}

impl Item {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Item::Empty,
            '#' => Item::Wall,
            'O' => Item::Box,
            '@' => Item::Robot,
            '[' => Item::BoxL,
            ']' => Item::BoxR,
            _ => panic!("Invalid thing"),
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Item::Empty => ".",
            Item::Wall => "#",
            Item::Box => "O",
            Item::Robot => "@",
            Item::BoxL => "[",
            Item::BoxR => "]",
        };
        write!(f, "{}", x)
    }
}

fn part1(input: &str) -> usize {
    let (map, directions) = split_double_newline_once(input);
    let mut grid = grid::Grid::from_str(map, Item::from_char);
    let mut robot_pos = grid.find(|x| x == &Item::Robot).unwrap();
    grid.replace(&robot_pos, Item::Empty);
    let directions = directions
        .lines()
        .join("")
        .chars()
        .map(dir_from_char)
        .collect_vec();
    for direction in directions.iter() {
        let next = direction.add_unsigned(&robot_pos, 1);
        let next = (next.0 as usize, next.1 as usize);
        match grid.get(next) {
            Some(Item::Empty) => {
                robot_pos = next;
            }
            Some(Item::Box) => {
                let start = (next.0 as isize, next.1 as isize);
                let mut curr = direction.add(&start, 1);
                loop {
                    match grid.get((curr.0 as usize, curr.1 as usize)) {
                        Some(Item::Empty) => {
                            let distance_to_empty =
                                (start.0 - curr.0).abs() + (start.1 - curr.1).abs() + 1;
                            for i in 1..distance_to_empty {
                                let pos = direction.add(&start, i as usize);
                                grid.replace(&(pos.0 as usize, pos.1 as usize), Item::Box);
                            }
                            grid.replace(&next, Item::Empty);
                            robot_pos = next;
                            break;
                        }
                        Some(Item::Box) => {
                            curr = direction.add(&curr, 1);
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    grid.find_many(|x| x == &Item::Box)
        .iter()
        .map(|x| x.0 * 100 + x.1)
        .sum()
}

fn part2(input: &str) -> usize {
    let input = &input
        .replace("#", "##")
        .replace("O", "[]")
        .replace(".", "..")
        .replace("@", "@.");
    let (map, directions) = split_double_newline_once(input);
    let mut grid = grid::Grid::from_str(map, Item::from_char);
    let mut robot_pos = grid.find(|x| x == &Item::Robot).unwrap();
    grid.replace(&robot_pos, Item::Empty);
    let directions = directions
        .lines()
        .join("")
        .chars()
        .map(dir_from_char)
        .collect_vec();
    for direction in directions.iter() {
        let next = direction.add_unsigned(&robot_pos, 1);
        let next = (next.0 as usize, next.1 as usize);
        match direction {
            CardinalDirection::North | CardinalDirection::South => match grid.get(next).unwrap() {
                Item::Empty => {
                    robot_pos = next;
                }
                Item::BoxL | Item::BoxR => {
                    let left = if grid.get(next).unwrap() == &Item::BoxL {
                        next
                    } else {
                        (next.0, next.1 - 1)
                    };
                    let mut bumped_lefts: Vec<(usize, usize)> = vec![];
                    let mut curr_bumped_lefts = vec![left];
                    'outer: loop {
                        let mut new_bumped_lefts = vec![];
                        for bumped_left in &curr_bumped_lefts {
                            let next_left = direction.add_unsigned(&bumped_left, 1);
                            let next_right =
                                direction.add_unsigned(&(bumped_left.0, bumped_left.1 + 1), 1);
                            match grid.get_virtual(next_left).unwrap() {
                                Item::Wall => break 'outer,
                                Item::BoxL => new_bumped_lefts
                                    .push((next_left.0 as usize, next_left.1 as usize)),
                                Item::BoxR => new_bumped_lefts
                                    .push((next_left.0 as usize, (next_left.1 - 1) as usize)),
                                _ => {}
                            }
                            match grid.get_virtual(next_right).unwrap() {
                                Item::Wall => break 'outer,
                                Item::BoxL => new_bumped_lefts
                                    .push((next_right.0 as usize, next_right.1 as usize)),
                                Item::BoxR => new_bumped_lefts
                                    .push((next_right.0 as usize, (next_right.1 - 1) as usize)),
                                _ => {}
                            }
                        }
                        if curr_bumped_lefts.is_empty() {
                            for bumped_left in &bumped_lefts {
                                let bumped_right = (bumped_left.0, bumped_left.1 + 1);
                                grid.replace(&bumped_left, Item::Empty);
                                grid.replace(&bumped_right, Item::Empty);
                            }
                            for bumped_left in bumped_lefts {
                                let bumped_right = (bumped_left.0, bumped_left.1 + 1);
                                let next_left = direction.add_unsigned(&bumped_left, 1);
                                let next_right = direction.add_unsigned(&bumped_right, 1);
                                grid.replace(
                                    &(next_left.0 as usize, next_left.1 as usize),
                                    Item::BoxL,
                                );
                                grid.replace(
                                    &(next_right.0 as usize, next_right.1 as usize),
                                    Item::BoxR,
                                );
                            }
                            robot_pos = next;
                            break 'outer;
                        }
                        bumped_lefts.extend_from_slice(curr_bumped_lefts.as_slice());
                        curr_bumped_lefts = new_bumped_lefts;
                    }
                }
                _ => {}
            },
            CardinalDirection::East | CardinalDirection::West => match grid.get(next).unwrap() {
                Item::Empty => {
                    robot_pos = next;
                }
                Item::BoxL | Item::BoxR => {
                    let start = (next.0 as isize, next.1 as isize);
                    let mut curr = direction.add(&start, 1);
                    loop {
                        match grid.get((curr.0 as usize, curr.1 as usize)) {
                            Some(Item::Empty) => {
                                let is_east = direction == &CardinalDirection::East;
                                let distance_to_empty =
                                    (start.0 - curr.0).abs() + (start.1 - curr.1).abs() + 1;
                                for i in (1..distance_to_empty).step_by(2) {
                                    let pos = direction.add(&start, i as usize);
                                    grid.replace(
                                        &(pos.0 as usize, pos.1 as usize),
                                        if is_east { Item::BoxL } else { Item::BoxR },
                                    );
                                    grid.replace(
                                        &(
                                            pos.0 as usize,
                                            if is_east {
                                                pos.1 as usize + 1
                                            } else {
                                                pos.1 as usize - 1
                                            },
                                        ),
                                        if is_east { Item::BoxR } else { Item::BoxL },
                                    );
                                }
                                grid.replace(&next, Item::Empty);
                                robot_pos = next;
                                break;
                            }
                            Some(Item::BoxL) | Some(Item::BoxR) => {
                                curr = direction.add(&curr, 1);
                            }
                            _ => {
                                break;
                            }
                        }
                    }
                }
                _ => {}
            },
        }
    }
    grid.find_many(|x| x == &Item::BoxL)
        .iter()
        .map(|x| x.0 * 100 + x.1)
        .sum()
}

fn main() {
    test_part1(|| part1(include_str!("./input/small.txt")), 2028);
    part1_test!(10092);
    part1_answer!(1487337);
    part2_test!(9021);
    part2_answer!(1521952);
}
