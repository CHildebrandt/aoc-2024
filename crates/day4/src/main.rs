enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn next_coord(&self, curr: (usize, usize)) -> Option<(usize, usize)> {
        let (y, x) = curr;
        let y = match self {
            Direction::North | Direction::NorthEast | Direction::NorthWest => {
                if y == 0 {
                    return None;
                }
                y - 1
            }
            Direction::South | Direction::SouthEast | Direction::SouthWest => y + 1,
            _ => y,
        };
        let x = match self {
            Direction::West | Direction::NorthWest | Direction::SouthWest => {
                if x == 0 {
                    return None;
                }
                x - 1
            }
            Direction::NorthEast | Direction::East | Direction::SouthEast => x + 1,
            _ => x,
        };
        Some((y, x))
    }

    fn list() -> &'static [Self] {
        &[
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ]
    }
}

fn has_pattern(
    map: &Vec<Vec<char>>,
    direction: &Direction,
    curr: (usize, usize),
    remaining: &[char],
) -> bool {
    match remaining.get(0) {
        Some(should_match) => match map.get(curr.0).and_then(|row| row.get(curr.1)) {
            Some(actual) => {
                if should_match == actual {
                    if remaining.len() == 1 {
                        return true;
                    }
                    match direction.next_coord(curr) {
                        Some(next) => has_pattern(map, direction, next, &remaining[1..]),
                        None => false,
                    }
                } else {
                    false
                }
            }
            None => false,
        },
        None => true,
    }
}

fn part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Direction::list().iter().fold(0, |acc, direction| {
        let mut acc = acc;
        for y in 0..input.len() {
            for x in 0..input[0].len() {
                if has_pattern(&input, direction, (y, x), &['X', 'M', 'A', 'S']) {
                    acc += 1;
                }
            }
        }
        acc
    })
}

fn part2(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut x_masses = 0;
    for y in 1..input.len() - 1 {
        for x in 1..input[0].len() - 1 {
            let curr = input.get(y).unwrap().get(x).unwrap();
            if *curr == 'A' {
                match input
                    .get(y - 1)
                    .and_then(|row| row.get(x - 1))
                    .zip(input.get(y + 1).and_then(|row| row.get(x + 1)))
                    .zip(input.get(y - 1).and_then(|row| row.get(x + 1)))
                    .zip(input.get(y + 1).and_then(|row| row.get(x - 1)))
                {
                    Some((((north_west, south_east), north_east), south_west)) => {
                        match (north_west, south_east) {
                            ('M', 'S') | ('S', 'M') => match (north_east, south_west) {
                                ('M', 'S') | ('S', 'M') => {
                                    x_masses += 1;
                                }
                                _ => {}
                            },
                            _ => {}
                        }
                    }
                    None => {}
                }
            }
        }
    }
    x_masses
}

fn main() {
    assert_eq!(part1(include_str!("./test")), 18);
    assert_eq!(part1(include_str!("./input")), 2358);
    assert_eq!(part2(include_str!("./test")), 9);
    assert_eq!(part2(include_str!("./input")), 1737);
}
