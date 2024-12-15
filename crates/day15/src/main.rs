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
}

impl Item {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Item::Empty,
            '#' => Item::Wall,
            'O' => Item::Box,
            '@' => Item::Robot,
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
        };
        write!(f, "{}", x)
    }
}

fn part1(input: &str) -> usize {
    let input = split_double_newline(input);
    let mut grid = grid::Grid::from_str(input[0], Item::from_char);
    let mut robot_pos = grid.find(|x| x == &Item::Robot).unwrap();
    grid.replace(&robot_pos, Item::Empty);
    let directions = input[1]
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
        .fold(0, |acc, x| acc + x.0 * 100 + x.1)
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    test_part1(|| part1(include_str!("./input/small.txt")), 2028);
    part1_test!(10092);
    part1_answer!(1487337);
    // part2_test!(9021);
    // part2_answer!(0);
}
