use std::ops::Neg;

pub type PositionVirtual = (isize, isize);

pub trait Direction: Sized {
    fn dydx(&self, distance: usize) -> PositionVirtual;
    fn all() -> &'static [Self];
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
