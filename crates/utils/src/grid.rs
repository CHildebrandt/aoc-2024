use num::{Integer, Signed, Unsigned};

use crate::direction::{CardinalDirection, Direction, OrdinalDirection, PositionVirtual};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;
use std::ops::{Add, Div, Index, IndexMut, Mul, Sub};

pub type Position = (usize, usize);

#[derive(Debug, Clone)]
pub struct Grid<T: Debug + Clone> {
    data: Vec<T>,
    height: usize,
    width: usize,
}

impl<T: Debug + Clone + Display> Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T: Debug + Clone> Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        &self.data[index.0 * self.width + index.1]
    }
}

impl<T: Debug + Clone> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        &mut self.data[index.0 * self.width + index.1]
    }
}

impl<T: Debug + Clone> Index<PositionVirtual> for Grid<T> {
    type Output = T;

    fn index(&self, index: PositionVirtual) -> &Self::Output {
        &self.data[index.0 as usize * self.width + index.1 as usize]
    }
}

impl<T: Debug + Clone> IndexMut<PositionVirtual> for Grid<T> {
    fn index_mut(&mut self, index: PositionVirtual) -> &mut Self::Output {
        &mut self.data[index.0 as usize * self.width + index.1 as usize]
    }
}

impl<T: Debug + Clone> Grid<T> {
    pub fn from_str(input: &str, f: impl Fn(char) -> T) -> Self {
        let mut data = Vec::with_capacity(input.len());
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

    pub fn get(&self, (y, x): Position) -> Option<&T> {
        if y < self.height && x < self.width {
            Some(&self.data[y * self.width + x])
        } else {
            None
        }
    }

    pub fn get_virtual(&self, pos: PositionVirtual) -> Option<&T> {
        if self.validate_position_virtual(pos) {
            Some(&self.data[pos.0 as usize * self.width + pos.1 as usize])
        } else {
            None
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    pub fn sub_grid(&self, height: usize, width: usize) -> SubGrid<T> {
        SubGrid {
            grid: self,
            height,
            width,
        }
    }

    pub fn is_corner(&self, (y, x): Position) -> bool {
        (y == 0 && x == 0)
            || (y == 0 && x == self.width - 1)
            || (y == self.height - 1 && x == 0)
            || (y == self.height - 1 && x == self.width - 1)
    }

    pub fn is_edge(&self, (y, x): Position) -> bool {
        y == 0 || x == 0 || y == self.height - 1 || x == self.width - 1
    }

    pub fn get_positions_where(&self, f: impl Fn(&T) -> bool) -> Vec<Position> {
        let mut positions = vec![];
        for (y, row) in self.iter_rows().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if f(cell) {
                    positions.push((y, x));
                }
            }
        }
        positions
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

    pub fn get_col(&self, x: usize) -> Option<Vec<T>> {
        if x < self.width {
            Some(
                (0..self.height)
                    .map(|y| self.data[y * self.width + x].clone())
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn iter_rows(&self) -> RowIter<'_, T> {
        RowIter::new(self)
    }

    pub fn find(&self, f: impl Fn(&T) -> bool) -> Option<Position> {
        for (y, row) in self.iter_rows().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if f(cell) {
                    return Some((y, x));
                }
            }
        }
        None
    }

    pub fn find_many(&self, f: impl Fn(&T) -> bool) -> Vec<Position> {
        let mut positions = vec![];
        for (y, row) in self.iter_rows().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if f(cell) {
                    positions.push((y, x));
                }
            }
        }
        positions
    }

    pub fn try_move_direction(
        &self,
        pos: &Position,
        direction: &CardinalDirection,
    ) -> Option<Position> {
        let new_pos = direction.add_unsigned(pos, 1);
        if self.validate_position_virtual(new_pos) {
            Some((new_pos.0 as usize, new_pos.1 as usize))
        } else {
            None
        }
    }

    pub fn move_if(
        &self,
        pos: &Position,
        direction: &CardinalDirection,
        f: impl Fn(&T) -> bool,
    ) -> Option<Position> {
        let new_pos = direction.add_unsigned(pos, 1);
        if self.validate_position_virtual(new_pos)
            && f(self.get((new_pos.0 as usize, new_pos.1 as usize)).unwrap())
        {
            Some((new_pos.0 as usize, new_pos.1 as usize))
        } else {
            None
        }
    }

    pub fn replace(&mut self, pos: &Position, value: T) {
        if self.validate_position(*pos) {
            self.data[pos.0 * self.width + pos.1] = value;
        }
    }

    pub fn replace_all_where(&mut self, f: impl Fn(&T) -> bool, value: T) {
        for cell in &mut self.data {
            if f(cell) {
                *cell = value.clone();
            }
        }
    }

    pub fn move_item(&mut self, from: Position, to: Position) {
        let item = self.get(from).unwrap().clone();
        self.replace(&from, self.get(to).unwrap().clone());
        self.replace(&to, item);
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

    fn neighbor_iter<'a, D: Direction>(&self, pos: &'a Position) -> NeighborIter<'_, 'a, T, D> {
        NeighborIter::new(self, pos)
    }

    fn neighbors<D: 'static + Direction>(&self, (y, x): Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        for direction in D::all() {
            let (dy, dx) = direction.dydx(1);
            let y = y as isize + dy;
            let x = x as isize + dx;
            if self.validate_position_virtual((y, x)) {
                neighbors.push((y as usize, x as usize));
            }
        }
        neighbors
    }

    pub fn neighbors_cardinal(&self, (y, x): Position) -> Vec<Position> {
        self.neighbors::<CardinalDirection>((y, x))
    }

    pub fn neighbors_ordinal(&self, (y, x): Position) -> Vec<Position> {
        self.neighbors::<OrdinalDirection>((y, x))
    }

    fn neighbors_virtual<D: 'static + Direction>(
        &self,
        pos: PositionVirtual,
    ) -> Vec<PositionVirtual> {
        let mut neighbors = Vec::new();
        for direction in D::all() {
            let (dy, dx) = direction.dydx(1);
            let y = pos.0 + dy;
            let x = pos.1 + dx;
            if self.validate_position_virtual((y, x)) {
                neighbors.push((y, x));
            }
        }
        neighbors
    }

    pub fn neighbors_cardinal_virtual(&self, pos: PositionVirtual) -> Vec<PositionVirtual> {
        self.neighbors_virtual::<CardinalDirection>(pos)
    }

    pub fn neighbors_ordinal_virtual(&self, pos: PositionVirtual) -> Vec<PositionVirtual> {
        self.neighbors_virtual::<OrdinalDirection>(pos)
    }

    pub fn distance_cardinal(&self, pos_a: Position, pos_b: Position) -> usize {
        let (y1, x1) = pos_a;
        let (y2, x2) = pos_b;
        y1.abs_diff(y2) + x1.abs_diff(x2)
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

    pub fn harmonics(&self, pos_a: Position, pos_b: Position) -> Vec<Position> {
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
            list.push((a.0 as usize, a.1 as usize));
            a = (a.0 + a_dy, a.1 + a_dx);
        }
        while self.validate_position_virtual(b) {
            list.push((b.0 as usize, b.1 as usize));
            b = (b.0 + b_dy, b.1 + b_dx);
        }
        // Not clear why we need to add the original positions?
        list.push(pos_a);
        list.push(pos_b);
        list
    }

    pub fn areas<F: Fn(&T, &T) -> bool>(
        &self,
        is_part_of_same_area: F,
    ) -> Vec<Vec<(Position, &T)>> {
        let mut areas = vec![];
        let mut visited = vec![vec![false; self.width]; self.height];
        for (y, row) in self.iter_rows().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if visited[y][x] {
                    continue;
                }
                let mut area = vec![];
                let mut stack = vec![(y, x)];
                while let Some((y, x)) = stack.pop() {
                    if visited[y][x] {
                        continue;
                    }
                    visited[y][x] = true;
                    let cell = &self.data[y * self.width + x];
                    area.push(((y, x), cell));
                    for neighbor in self.neighbors_cardinal((y, x)) {
                        let (ny, nx) = neighbor;
                        if !visited[ny][nx]
                            && is_part_of_same_area(cell, &self.data[ny * self.width + nx])
                        {
                            stack.push(neighbor);
                        }
                    }
                }
                areas.push(area);
            }
        }
        areas
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

    pub fn clear(&mut self) {
        for cell in &mut self.data {
            *cell = T::default();
        }
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

pub struct ColIter<'a, T: Debug + Clone> {
    grid: &'a Grid<T>,
    i: usize,
}

impl<'a, T: Debug + Clone> ColIter<'a, T> {
    pub fn new(grid: &'a Grid<T>) -> Self {
        Self { grid, i: 0 }
    }
}

impl<'a, T: Debug + Clone> Iterator for ColIter<'a, T> {
    type Item = Vec<T>; // TODO: &[T]

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.grid.get_col(self.i);
        self.i += 1;
        next
    }
}

pub struct NeighborIter<'a, 'b, T: Debug + Clone, D: Direction> {
    grid: &'a Grid<T>,
    pos: &'b Position,
    i: usize,
    __direction: PhantomData<D>,
}

impl<'a, 'b, T: Debug + Clone, D: Direction> NeighborIter<'a, 'b, T, D> {
    pub fn new(grid: &'a Grid<T>, pos: &'b Position) -> Self {
        Self {
            grid,
            pos,
            i: 0,
            __direction: PhantomData,
        }
    }
}

impl<'a, 'b, T: Debug + Clone, D: Direction + 'static> Iterator for NeighborIter<'a, 'b, T, D> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(dir) = D::all().get(self.i) {
            self.i += 1;
            let pos = dir.add_unsigned(self.pos, 1);
            if let Some(t) = self.grid.get_virtual(pos) {
                return Some(((pos.0 as usize, pos.1 as usize), t));
            }
        }
        None
    }
}

pub struct SubGrid<'a, T: Debug + Clone> {
    grid: &'a Grid<T>,
    height: usize,
    width: usize,
}

pub struct GridPos<I: Integer>(pub I, pub I);

impl<I: Integer> GridPos<I> {
    pub fn new(y: I, x: I) -> Self {
        Self(y, x)
    }
}

impl<I: Integer + std::str::FromStr> GridPos<I> {
    pub fn from_str(input: &str) -> Result<Self, I::Err> {
        let mut parts = input.split(',');
        let y = parts.next().unwrap().parse()?;
        let x = parts.next().unwrap().parse()?;
        Ok(Self(y, x))
    }
}

impl<I: Integer + Copy> GridPos<I> {
    pub fn flip(&self) -> Self {
        Self(self.1, self.0)
    }
}

impl<I: Integer + Clone> Clone for GridPos<I> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}
impl<I: Integer + Copy> Copy for GridPos<I> {}

impl<I: Integer + Signed + Copy> GridPos<I> {
    pub fn try_unsign<U: Integer + Unsigned + std::convert::From<I>>(&self) -> Option<GridPos<U>> {
        if self.0.is_negative() || self.1.is_negative() {
            None
        } else {
            Some(GridPos(self.0.into(), self.1.into()))
        }
    }
}

impl<T: Debug + Clone> Index<GridPos<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, index: GridPos<usize>) -> &Self::Output {
        &self.data[index.0 * self.height + index.1]
    }
}

impl<T: Debug + Clone> IndexMut<GridPos<usize>> for Grid<T> {
    fn index_mut(&mut self, index: GridPos<usize>) -> &mut Self::Output {
        &mut self.data[index.0 * self.height + index.1]
    }
}

impl<T: Debug + Clone> Index<&GridPos<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, index: &GridPos<usize>) -> &Self::Output {
        &self.data[index.0 * self.height + index.1]
    }
}

impl<T: Debug + Clone> IndexMut<&GridPos<usize>> for Grid<T> {
    fn index_mut(&mut self, index: &GridPos<usize>) -> &mut Self::Output {
        &mut self.data[index.0 * self.height + index.1]
    }
}

impl<I: Integer> Add for GridPos<I> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl<I: Integer> Sub for GridPos<I> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl<I: Integer> Mul for GridPos<I> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }
}

impl<I: Integer> Div for GridPos<I> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0, self.1 / other.1)
    }
}

impl<I: Integer> From<(I, I)> for GridPos<I> {
    fn from(pos: (I, I)) -> Self {
        Self(pos.0, pos.1)
    }
}

impl<I: Integer + Copy> From<&(I, I)> for GridPos<I> {
    fn from(pos: &(I, I)) -> Self {
        Self(pos.0, pos.1)
    }
}

impl<I: Integer> From<GridPos<I>> for (I, I) {
    fn from(pos: GridPos<I>) -> Self {
        (pos.0, pos.1)
    }
}

impl<I: Integer + Copy> From<&GridPos<I>> for (I, I) {
    fn from(pos: &GridPos<I>) -> Self {
        (pos.0, pos.1)
    }
}

pub trait Obstructs {
    fn obstructs(&self) -> bool;
}

impl<T: Debug + Clone + Obstructs> Grid<T> {
    pub fn obstructed_positions(&self) -> Vec<Position> {
        self.get_positions_where(|cell| cell.obstructs())
    }

    pub fn unobstructed_positions(&self) -> Vec<Position> {
        self.get_positions_where(|cell| !cell.obstructs())
    }

    pub fn neighbor_iter_unobstructed<'a, D: Direction + 'static>(
        &'a self,
        pos: &'a Position,
    ) -> impl Iterator<Item = (Position, &T)> + 'a {
        self.neighbor_iter::<D>(pos).filter(|(_, t)| !t.obstructs())
    }

    pub fn astar<D: Direction + 'static>(
        &self,
        start: &Position,
        end: &Position,
    ) -> Option<(Vec<Position>, usize)> {
        pathfinding::prelude::astar(
            start,
            |pos| {
                self.neighbor_iter_unobstructed::<D>(pos)
                    .map(|(n, _)| (n, 1))
                    .collect::<Vec<_>>()
            },
            |pos| self.distance_cardinal(*pos, *end),
            |pos| pos == end,
        )
    }

    pub fn astar_cardinal(
        &self,
        start: &Position,
        end: &Position,
    ) -> Option<(Vec<Position>, usize)> {
        self.astar::<CardinalDirection>(start, end)
    }

    pub fn astar_ordinal(
        &self,
        start: &Position,
        end: &Position,
    ) -> Option<(Vec<Position>, usize)> {
        self.astar::<OrdinalDirection>(start, end)
    }
}
