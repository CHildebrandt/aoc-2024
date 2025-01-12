use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

use direction::PositionVirtual;
use grid::{Grid, Position};
use itertools::Itertools;
use utils::*;

const TEST: &str = include_str!("./input/test.txt");
const INPUT: &str = include_str!("./input/input.txt");

#[derive(Debug)]
struct Robot {
    pos: Position,
    vel: PositionVirtual,
}

impl Robot {
    fn from_str(input: &str) -> Self {
        let (pos, vel) = input
            .split_once(" ")
            .map(|(pos, vel)| {
                let mut pos_split = pos.split("=").nth(1).unwrap().split(",");
                let mut vel_split = vel.split("=").nth(1).unwrap().split(",");
                (
                    (
                        pos_split.next().unwrap().parse().unwrap(),
                        pos_split.next().unwrap().parse().unwrap(),
                    ),
                    (
                        vel_split.next().unwrap().parse().unwrap(),
                        vel_split.next().unwrap().parse().unwrap(),
                    ),
                )
            })
            .map(|(pos, vel)| ((pos.1, pos.0), (vel.1, vel.0)))
            .unwrap();
        Self { pos, vel }
    }

    fn next_pos<T: Debug + Clone>(&mut self, grid: &Grid<T>) {
        let (h, w) = grid.size();
        let (h, w) = (h as isize, w as isize);
        let (curr_y, curr_x) = (self.pos.0 as isize, self.pos.1 as isize);
        let (vel_y, vel_x) = self.vel;
        let mut y = curr_y + vel_y;
        let mut x = curr_x + vel_x;
        while y < 0 {
            y += h;
        }
        while y >= h {
            y -= h;
        }
        while x < 0 {
            x += w;
        }
        while x >= w {
            x -= w;
        }
        self.pos = (y as usize, x as usize);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Quadrant {
    None,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

fn part1(input: &str, width: usize, height: usize) -> usize {
    let grid = Grid::blank(height, width, '.');
    let mut robots = input.lines().map(Robot::from_str).collect_vec();
    for _ in 0..100 {
        robots.iter_mut().for_each(|robot| robot.next_pos(&grid));
    }
    let max_y = height - 1;
    let max_x = width - 1;
    let mid_y = max_y / 2;
    let mid_x = max_x / 2;
    let mut mapped = HashMap::<Quadrant, Vec<Robot>>::new();
    for robot in robots {
        let quadrant = if robot.pos.0 == mid_y || robot.pos.1 == mid_x {
            Quadrant::None
        } else if robot.pos.0 < mid_y && robot.pos.1 < mid_x {
            Quadrant::TopLeft
        } else if robot.pos.0 < mid_y && robot.pos.1 > mid_x {
            Quadrant::TopRight
        } else if robot.pos.0 > mid_y && robot.pos.1 < mid_x {
            Quadrant::BottomLeft
        } else {
            Quadrant::BottomRight
        };
        mapped.entry(quadrant).or_insert_with(Vec::new).push(robot);
    }
    mapped.iter().fold(0, |factor, (quadrant, group)| {
        if quadrant == &Quadrant::None {
            factor
        } else if factor == 0 {
            group.len()
        } else {
            factor * group.len()
        }
    })
}

fn part2(input: &str) -> usize {
    let grid = Grid::blank(103, 101, '.');
    let mut robots = input.lines().map(Robot::from_str).collect_vec();
    let mut checked = HashSet::new();
    let mut pos_set = HashSet::new();
    for sec in 1..100_000 {
        checked.clear();
        pos_set.clear();
        robots.iter_mut().for_each(|robot| robot.next_pos(&grid));
        pos_set.extend(robots.iter().map(|robot| robot.pos));
        let neighboring = robots
            .iter()
            .map(|robot| {
                checked.insert(robot.pos);
                grid.neighbors_ordinal(robot.pos)
                    .iter()
                    .filter(|pos| !checked.contains(pos) && pos_set.contains(pos))
                    .count()
            })
            .sum::<usize>();
        let neighboring = neighboring / 2;
        if neighboring > robots.len() / 2 {
            return sec;
        }
    }
    0
}

fn main() {
    test_part1(|| part1(TEST, 11, 7), 12);
    answer_part1(|| part1(INPUT, 101, 103), 216772608);
    part2_answer!(6888);
}
