use std::collections::HashSet;

#[derive(Clone, PartialEq)]
enum Floor {
    Empty,
    Obstruction,
}

impl Floor {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Floor::Empty),
            '#' => Some(Floor::Obstruction),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn from_char(c: char) -> Option<Self> {
        match c {
            '^' => Some(Direction::North),
            'v' => Some(Direction::South),
            '<' => Some(Direction::West),
            '>' => Some(Direction::East),
            _ => None,
        }
    }
}

#[derive(Clone)]
struct Guard {
    y: usize,
    x: usize,
    direction: Direction,
}

impl Guard {
    fn new(y: usize, x: usize, c: char) -> Option<Self> {
        Direction::from_char(c).map(|direction| Self { y, x, direction })
    }

    fn position(&self) -> (usize, usize) {
        (self.y, self.x)
    }

    fn key(&self) -> (usize, usize, Direction) {
        (self.y, self.x, self.direction.clone())
    }

    fn next(&mut self, map: &Vec<Vec<Floor>>) -> Option<(usize, usize, Direction)> {
        if (self.y == 0 && self.direction == Direction::North)
            || (self.y == map.len() - 1 && self.direction == Direction::South)
            || (self.x == 0 && self.direction == Direction::West)
            || (self.x == map[0].len() - 1 && self.direction == Direction::East)
        {
            None
        } else {
            let next_y = match self.direction {
                Direction::North => self.y - 1,
                Direction::South => self.y + 1,
                _ => self.y,
            };
            let next_x = match self.direction {
                Direction::East => self.x + 1,
                Direction::West => self.x - 1,
                _ => self.x,
            };
            match map[next_y][next_x] {
                Floor::Obstruction => {
                    self.direction = self.direction.next();
                }
                Floor::Empty => {
                    self.y = next_y;
                    self.x = next_x;
                }
            }
            Some(self.key())
        }
    }
}

fn parse(input: &str) -> (Vec<Vec<Floor>>, Guard) {
    let (map, guard) =
        input
            .lines()
            .enumerate()
            .fold((vec![], None), |(mut map, mut guard), (y, line)| {
                let mut row = vec![];
                for (x, c) in line.chars().enumerate() {
                    if let Some(floor) = Floor::from_char(c) {
                        row.push(floor);
                    } else if let Some(g) = Guard::new(y, x, c) {
                        guard = Some(g);
                        row.push(Floor::Empty);
                    }
                }
                map.push(row);
                (map, guard)
            });
    (map, guard.unwrap())
}

fn part1(input: &str) -> usize {
    let (map, mut guard) = parse(input);
    let mut visited = HashSet::new();
    visited.insert(guard.position());
    while let Some((y, x, _)) = guard.next(&map) {
        visited.insert((y, x));
    }
    visited.len()
}

fn part2(input: &str) -> usize {
    let (mut map, guard) = parse(input);
    let mut visited = HashSet::new();
    let mut guard_check = guard.clone();
    while let Some((y, x, _)) = guard_check.next(&map) {
        visited.insert((y, x));
    }
    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            // Skip those coordinates that are not in the guard's path
            if !visited.contains(&(y, x)) {
                continue;
            }
            if map[y][x] == Floor::Empty {
                let mut guard = guard.clone();
                // Mutate (performance optimization compared to cloning the map)
                map[y][x] = Floor::Obstruction;
                let mut visited = HashSet::new();
                visited.insert(guard.key());
                while let Some(key) = guard.next(&map) {
                    if visited.contains(&key) {
                        count += 1;
                        break;
                    }
                    visited.insert(key);
                }
                // Undo mutation
                map[y][x] = Floor::Empty;
            }
        }
    }
    count
}

fn main() {
    assert_eq!(part1(include_str!("./test")), 41);
    assert_eq!(part1(include_str!("./input")), 5153);
    assert_eq!(part2(include_str!("./test")), 6);
    assert_eq!(part2(include_str!("./input")), 1711);
}
