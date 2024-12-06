use std::collections::HashSet;

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
}

struct Guard {
    y: usize,
    x: usize,
    direction: Direction,
}

impl Guard {
    fn new(y: usize, x: usize, c: char) -> Option<Self> {
        match c {
            '^' => Some(Guard {
                y,
                x,
                direction: Direction::North,
            }),
            'v' => Some(Guard {
                y,
                x,
                direction: Direction::South,
            }),
            '<' => Some(Guard {
                y,
                x,
                direction: Direction::West,
            }),
            '>' => Some(Guard {
                y,
                x,
                direction: Direction::East,
            }),
            _ => None,
        }
    }

    fn position(&self) -> (usize, usize) {
        (self.y, self.x)
    }

    fn next(&mut self, map: &Vec<Vec<Floor>>) -> Option<(usize, usize)> {
        match self.direction {
            Direction::North => {
                if self.y == 0 {
                    None
                } else {
                    match map[self.y - 1][self.x] {
                        Floor::Obstruction => {
                            self.direction = self.direction.next();
                        }
                        Floor::Empty => {
                            self.y -= 1;
                        }
                    }
                    Some(self.position())
                }
            }
            Direction::South => {
                if self.y == map.len() - 1 {
                    None
                } else {
                    match map[self.y + 1][self.x] {
                        Floor::Obstruction => {
                            self.direction = self.direction.next();
                        }
                        Floor::Empty => {
                            self.y += 1;
                        }
                    }
                    Some(self.position())
                }
            }
            Direction::East => {
                if self.x == map[0].len() - 1 {
                    None
                } else {
                    match map[self.y][self.x + 1] {
                        Floor::Obstruction => {
                            self.direction = self.direction.next();
                        }
                        Floor::Empty => {
                            self.x += 1;
                        }
                    }
                    Some(self.position())
                }
            }
            Direction::West => {
                if self.x == 0 {
                    None
                } else {
                    match map[self.y][self.x - 1] {
                        Floor::Obstruction => {
                            self.direction = self.direction.next();
                        }
                        Floor::Empty => {
                            self.x -= 1;
                        }
                    }
                    Some(self.position())
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
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
    let mut guard = guard.unwrap();
    let mut visited = HashSet::new();
    visited.insert(guard.position());
    while let Some(position) = guard.next(&map) {
        visited.insert(position);
    }
    visited.len()
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    assert_eq!(part1(include_str!("./test")), 41);
    assert_eq!(part1(include_str!("./input")), 0);
    // assert_eq!(part2(include_str!("./test")), 0);
    // assert_eq!(part2(include_str!("./input")), 0);
}
