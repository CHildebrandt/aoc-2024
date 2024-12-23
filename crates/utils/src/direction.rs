use std::{cmp::Ordering, ops::Neg};

pub type PositionVirtual = (isize, isize);

pub trait Direction: Sized {
    fn dydx(&self, distance: usize) -> PositionVirtual;
    fn all() -> &'static [Self];

    fn add(&self, pos: &PositionVirtual, distance: usize) -> PositionVirtual {
        let (dy, dx) = self.dydx(distance);
        (pos.0 + dy, pos.1 + dx)
    }

    fn add_unsigned(&self, pos: &(usize, usize), distance: usize) -> PositionVirtual {
        let (dy, dx) = self.dydx(distance);
        (pos.0 as isize + dy, pos.1 as isize + dx)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardinalDirection {
    North,
    South,
    East,
    West,
}

impl Direction for CardinalDirection {
    fn all() -> &'static [Self] {
        &[Self::North, Self::South, Self::East, Self::West]
    }

    fn dydx(&self, distance: usize) -> PositionVirtual {
        let distance = distance as isize;
        match self {
            Self::North => (distance.neg(), 0),
            Self::South => (distance, 0),
            Self::East => (0, distance),
            Self::West => (0, distance.neg()),
        }
    }
}

impl CardinalDirection {
    pub fn from_diff(diff: PositionVirtual) -> Option<Self> {
        match diff {
            (0, 0) => None,
            (0, x) if x > 0 => Some(Self::East),
            (0, x) if x < 0 => Some(Self::West),
            (y, 0) if y > 0 => Some(Self::South),
            (y, 0) if y < 0 => Some(Self::North),
            _ => None,
        }
    }

    pub fn from_cmp(y_cmp: Ordering, x_cmp: Ordering) -> Option<Self> {
        match (y_cmp, x_cmp) {
            (Ordering::Less, Ordering::Equal) => Some(Self::North),
            (Ordering::Greater, Ordering::Equal) => Some(Self::South),
            (Ordering::Equal, Ordering::Less) => Some(Self::West),
            (Ordering::Equal, Ordering::Greater) => Some(Self::East),
            _ => None,
        }
    }

    pub fn is_180(&self, other: &Self) -> bool {
        match self {
            Self::North => other == &Self::South,
            Self::South => other == &Self::North,
            Self::East => other == &Self::West,
            Self::West => other == &Self::East,
        }
    }

    pub fn is_90cw(&self, other: &Self) -> bool {
        match self {
            Self::North => other == &Self::East,
            Self::South => other == &Self::West,
            Self::East => other == &Self::South,
            Self::West => other == &Self::North,
        }
    }

    pub fn is_90ccw(&self, other: &Self) -> bool {
        match self {
            Self::North => other == &Self::West,
            Self::South => other == &Self::East,
            Self::East => other == &Self::North,
            Self::West => other == &Self::South,
        }
    }
}

impl std::fmt::Display for CardinalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = match self {
            Self::North => "N",
            Self::South => "S",
            Self::East => "E",
            Self::West => "W",
        };
        write!(f, "{}", x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrdinalDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction for OrdinalDirection {
    fn all() -> &'static [Self] {
        &[
            Self::North,
            Self::NorthEast,
            Self::East,
            Self::SouthEast,
            Self::South,
            Self::SouthWest,
            Self::West,
            Self::NorthWest,
        ]
    }

    fn dydx(&self, distance: usize) -> PositionVirtual {
        let distance = distance as isize;
        let dy = match self {
            Self::North | Self::NorthEast | Self::NorthWest => distance.neg(),
            Self::East | Self::West => 0,
            _ => distance,
        };
        let dx = match self {
            Self::West | Self::SouthWest | Self::NorthWest => distance.neg(),
            Self::North | Self::South => 0,
            _ => distance,
        };
        (dy, dx)
    }
}

impl OrdinalDirection {
    pub fn from_diff(diff: PositionVirtual) -> Option<Self> {
        match diff {
            (0, 0) => None,
            (0, x) if x > 0 => Some(Self::East),
            (0, x) if x < 0 => Some(Self::West),
            (y, 0) if y > 0 => Some(Self::South),
            (y, 0) if y < 0 => Some(Self::North),
            (y, x) if y > 0 && x > 0 => Some(Self::SouthEast),
            (y, x) if y > 0 && x < 0 => Some(Self::SouthWest),
            (y, x) if y < 0 && x > 0 => Some(Self::NorthEast),
            (y, x) if y < 0 && x < 0 => Some(Self::NorthWest),
            _ => None,
        }
    }
}
