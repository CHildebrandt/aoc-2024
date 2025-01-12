use direction::{CardinalDirection, Direction};
use utils::*;

#[derive(Clone, PartialEq, Debug)]
enum Floor {
    Empty,
    Obstruction,
}

impl From<char> for Floor {
    fn from(c: char) -> Self {
        match c {
            '#' => Floor::Obstruction,
            _ => Floor::Empty,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Guard {
    y: usize,
    x: usize,
    direction: CardinalDirection,
}

impl Guard {
    fn new(y: usize, x: usize, direction: CardinalDirection) -> Self {
        Self { y, x, direction }
    }

    fn position(&self) -> (usize, usize) {
        (self.y, self.x)
    }

    fn exiting(&mut self, grid: &grid::Grid<Floor>) -> bool {
        if (self.y == 0 && self.direction == CardinalDirection::North)
            || (self.y == grid.height() - 1 && self.direction == CardinalDirection::South)
            || (self.x == 0 && self.direction == CardinalDirection::West)
            || (self.x == grid.width() - 1 && self.direction == CardinalDirection::East)
        {
            true
        } else {
            let next = self.direction.add_unsigned(&(self.y, self.x), 1);
            match grid.get_virtual(next).unwrap() {
                Floor::Obstruction => {
                    self.direction = match self.direction {
                        CardinalDirection::North => CardinalDirection::East,
                        CardinalDirection::East => CardinalDirection::South,
                        CardinalDirection::South => CardinalDirection::West,
                        CardinalDirection::West => CardinalDirection::North,
                    };
                }
                Floor::Empty => {
                    self.y = next.0 as usize;
                    self.x = next.1 as usize;
                }
            }
            false
        }
    }
}

fn parse(input: &str) -> (grid::Grid<Floor>, Guard) {
    let grid = grid::Grid::char_grid(input);
    let guard = grid
        .iter()
        .find_map(|(pos, c)| {
            match c {
                '^' => Some(CardinalDirection::North),
                'v' => Some(CardinalDirection::South),
                '<' => Some(CardinalDirection::West),
                '>' => Some(CardinalDirection::East),
                _ => None,
            }
            .map(|direction| Guard::new(pos.0, pos.1, direction))
        })
        .unwrap();
    let grid = grid.map(|c| Floor::from(*c));
    (grid, guard)
}

fn part1(input: &str) -> usize {
    let (grid, mut guard) = parse(input);
    let mut visited = fxhash::FxHashSet::default();
    visited.insert(guard.position());
    while !guard.exiting(&grid) {
        visited.insert(guard.position());
    }
    visited.len()
}

fn part2(input: &str) -> usize {
    let (grid, guard) = parse(input);
    let mut initial_visited = fxhash::FxHashSet::default();
    let mut guard_check = guard.clone();
    while !guard_check.exiting(&grid) {
        initial_visited.insert(guard_check.position());
    }
    let mut visited = fxhash::FxHashSet::default();
    let mut mutable_grid = grid.clone();
    grid.iter()
        .filter(|(pos, floor)| initial_visited.contains(&pos) && **floor == Floor::Empty)
        .filter(|(pos, _)| {
            visited.clear();
            let mut guard = guard.clone();
            mutable_grid[*pos] = Floor::Obstruction;
            visited.insert(guard.clone());
            while !guard.exiting(&mutable_grid) {
                if visited.contains(&guard) {
                    mutable_grid[*pos] = Floor::Empty;
                    return true;
                }
                visited.insert(guard.clone());
            }
            mutable_grid[*pos] = Floor::Empty;
            false
        })
        .count()
}

fn main() {
    part1_test!(41);
    part1_answer!(5153);
    part2_test!(6);
    part2_answer!(1711);
}
