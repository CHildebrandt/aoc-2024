use grid::Position;
use utils::*;

fn parse_pos(input: &str) -> Position {
    let input = input
        .split(": ")
        .nth(1)
        .unwrap()
        .split(", ")
        .map(|l| l[2..].parse().unwrap())
        .collect::<Vec<_>>();
    (input[0], input[1])
}

struct ClawMachine {
    a_button: Position,
    b_button: Position,
    prize: Position,
}

impl ClawMachine {
    fn new(a_button: Position, b_button: Position, prize: Position) -> Self {
        Self {
            a_button,
            b_button,
            prize,
        }
    }

    fn with_prize_add(mut self, added: usize) -> Self {
        self.prize.0 += added;
        self.prize.1 += added;
        self
    }

    fn from_str(input: &str) -> Self {
        let mut lines = input.lines();
        let a_button = parse_pos(lines.next().unwrap());
        let b_button = parse_pos(lines.next().unwrap());
        let prize = parse_pos(lines.next().unwrap());
        Self::new(a_button, b_button, prize)
    }

    fn calculate_wins(&self) -> Option<usize> {
        let (py, px) = (self.prize.0 as isize, self.prize.1 as isize);
        let (ay, ax) = (self.a_button.0 as isize, self.a_button.1 as isize);
        let (by, bx) = (self.b_button.0 as isize, self.b_button.1 as isize);

        let denom = by * ax - bx * ay;
        if denom == 0 {
            None
        } else {
            let min_b_presses = (py * ax - px * ay) / denom;
            let min_a_presses = if ax != 0 {
                (px - min_b_presses * bx) / ax
            } else if ay != 0 {
                (py - min_b_presses * by) / ay
            } else {
                return None;
            };

            if min_a_presses * ax + min_b_presses * bx == px
                && min_a_presses * ay + min_b_presses * by == py
            {
                Some(min_a_presses as usize * 3 + min_b_presses as usize)
            } else {
                None
            }
        }
    }
}

fn part1(input: &str) -> usize {
    split_double_newline(input)
        .iter()
        .map(|x| ClawMachine::from_str(x))
        .filter_map(|machine| machine.calculate_wins())
        .sum()
}

fn part2(input: &str) -> usize {
    split_double_newline(input)
        .iter()
        .map(|x| ClawMachine::from_str(x).with_prize_add(10_000_000_000_000))
        .filter_map(|machine| machine.calculate_wins())
        .sum()
}

fn main() {
    part1_test!(480);
    part1_answer!(29517);
    part2_answer!(103570327981381);
}
