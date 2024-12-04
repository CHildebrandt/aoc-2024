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
    let get = |y: usize, x: usize| -> Option<&char> { input.get(y).and_then(|row| row.get(x)) };
    input.iter().enumerate().skip(1).fold(0, |acc, (y, row)| {
        acc + row.iter().enumerate().skip(1).fold(0, |acc, (x, curr)| {
            if curr == &'A' {
                if let Some((((north_west, south_east), north_east), south_west)) =
                    get(y - 1, x - 1)
                        .zip(get(y + 1, x + 1))
                        .zip(get(y - 1, x + 1))
                        .zip(get(y + 1, x - 1))
                {
                    if matches!((north_west, south_east), ('M', 'S') | ('S', 'M'))
                        && matches!((north_east, south_west), ('M', 'S') | ('S', 'M'))
                    {
                        return acc + 1;
                    }
                }
            }
            acc
        })
    })
}

fn main() {
    assert_eq!(part1(include_str!("./test")), 18);
    assert_eq!(part1(include_str!("./input")), 2358);
    assert_eq!(part2(include_str!("./test")), 9);
    assert_eq!(part2(include_str!("./input")), 1737);
}
