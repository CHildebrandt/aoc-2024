use crate::direction::PositionVirtual;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;

pub type Position = (usize, usize);

#[derive(Debug, Clone)]
pub struct Grid<T: Debug + Clone> {
    data: Vec<T>,
    height: usize,
    width: usize,
}

impl<T: Debug + Clone> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            for cell in row {
                write!(f, "{:?}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Debug + Clone> Grid<T> {
    pub fn from_str(input: &str, f: impl Fn(char) -> T) -> Self {
        let mut data = vec![];
        let mut height = 0;
        let mut width = 0;
        for (y, line) in input.lines().enumerate() {
            if y == 0 {
                width = line.len();
            } else {
                assert_eq!(line.len(), width, "Inconsistent row length!");
            }
            let mut row = vec![];
            for c in line.chars() {
                row.push(f(c));
            }
            data.extend(row);
            height += 1;
        }
        assert_ne!(height, 0, "Empty grid!");
        assert_ne!(width, 0, "Empty grid!");
        Self {
            data,
            width,
            height,
        }
    }

    pub fn blank(height: usize, width: usize, def: T) -> Self {
        Self {
            data: vec![def; width * height],
            width,
            height,
        }
    }

    pub fn validate_position(&self, pos: Position) -> bool {
        let (y, x) = pos;
        y < self.height && x < self.width
    }

    pub fn validate_position_virtual(&self, pos: PositionVirtual) -> bool {
        let (y, x) = pos;
        y >= 0 && y < self.height as isize && x >= 0 && x < self.width as isize
    }

    pub fn filter_positions(&self, positions: &[Position]) -> Vec<Position> {
        positions
            .iter()
            .filter(|&pos| self.validate_position(*pos))
            .map(|&pos| pos)
            .collect()
    }

    pub fn filter_positions_virtual(&self, positions: &[PositionVirtual]) -> Vec<Position> {
        positions
            .iter()
            .filter(|&pos| self.validate_position_virtual(*pos))
            .map(|&pos| (pos.0 as usize, pos.1 as usize))
            .collect()
    }

    pub fn get_row(&self, y: usize) -> Option<&[T]> {
        if y < self.height {
            Some(&self.data[y * self.width..(y + 1) * self.width])
        } else {
            None
        }
    }

    pub fn iter_rows(&self) -> RowIter<'_, T> {
        RowIter::new(self)
    }

    pub fn group_by<K: Eq + Hash, F: Fn(&T) -> K>(&self, f: F) -> HashMap<K, Vec<(Position, &T)>> {
        let mut groups = HashMap::new();
        for (y, row) in self.iter_rows().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                groups.entry(f(cell)).or_insert(vec![]).push(((y, x), cell));
            }
        }
        groups
    }

    // TODO: naming
    pub fn get_outer_diff_positions(
        &self,
        pos_a: Position,
        pos_b: Position,
    ) -> (PositionVirtual, PositionVirtual) {
        // TODO: return Err instead
        if !self.validate_position(pos_a) || !self.validate_position(pos_b) {
            panic!("Invalid positions!");
        }
        let a_y = pos_a.0 as isize;
        let a_x = pos_a.1 as isize;
        let b_y = pos_b.0 as isize;
        let b_x = pos_b.1 as isize;
        (
            (a_y + (a_y - b_y), a_x + (a_x - b_x)),
            (b_y + (b_y - a_y), b_x + (b_x - a_x)),
        )
    }

    // TODO: naming
    pub fn get_outer_diff_positions_multi(
        &self,
        pos_a: Position,
        pos_b: Position,
    ) -> Vec<PositionVirtual> {
        // TODO: return Err instead
        if !self.validate_position(pos_a) || !self.validate_position(pos_b) {
            panic!("Invalid positions!");
        }
        assert_ne!(pos_a, pos_b, "Same positions for a and b!");
        let mut list = vec![];
        let a_y = pos_a.0 as isize;
        let a_x = pos_a.1 as isize;
        let b_y = pos_b.0 as isize;
        let b_x = pos_b.1 as isize;
        let a_dy = a_y - b_y;
        let a_dx = a_x - b_x;
        let b_dy = b_y - a_y;
        let b_dx = b_x - a_x;
        let mut a = (a_y + a_dy, a_x + a_dx);
        let mut b = (b_y + b_dy, b_x + b_dx);
        while self.validate_position_virtual(a) {
            list.push(a);
            a = (a.0 + a_dy, a.1 + a_dx);
        }
        while self.validate_position_virtual(b) {
            list.push(b);
            b = (b.0 + b_dy, b.1 + b_dx);
        }
        list.push((a_y, a_x));
        list.push((b_y, b_x));
        list
    }
}

impl Grid<char> {
    pub fn char_grid(input: &str) -> Self {
        Self::from_str(input, |c| c)
    }
}

impl<T: Debug + Clone + Default> Grid<T> {
    pub fn defaulted(height: usize, width: usize) -> Self {
        Self::blank(height, width, T::default())
    }
}

impl<T: Debug + Clone + Eq + Hash> Grid<T> {
    pub fn group_by_cell_value(&self) -> HashMap<T, Vec<Position>> {
        self.group_by(|cell| cell.clone())
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().map(|(pos, _)| pos).collect()))
            .collect()
    }
}

pub struct RowIter<'a, T: Debug + Clone> {
    grid: &'a Grid<T>,
    i: usize,
}

impl<'a, T: Debug + Clone> RowIter<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self { grid, i: 0 }
    }
}

impl<'a, T: Debug + Clone> Iterator for RowIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.grid.get_row(self.i);
        self.i += 1;
        next
    }
}
