use utils::*;

struct Coord {
    x: usize,
    y: usize,
}

struct ClawMachine {
    a_button: Coord,
    b_button: Coord,
    prize: Coord,
}

impl ClawMachine {
    fn new(a_button: Coord, b_button: Coord, prize: Coord) -> Self {
        Self {
            a_button,
            b_button,
            prize,
        }
    }

    fn parse_line(line: &str) -> Coord {
        let line = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|l| l[2..].parse().unwrap())
            .collect::<Vec<_>>();
        Coord {
            x: line[0],
            y: line[1],
        }
    }

    fn from_str(input: &str) -> Self {
        let mut lines = input.lines();
        let a_button = Self::parse_line(lines.next().unwrap());
        let b_button = Self::parse_line(lines.next().unwrap());
        let prize = Self::parse_line(lines.next().unwrap());
        Self::new(a_button, b_button, prize)
    }
}

fn part1(input: &str) -> usize {
    let input = split_double_newline(input)
        .iter()
        .map(|x| ClawMachine::from_str(x))
        .collect::<Vec<_>>();
    0
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    part1_test!(0);
    // part1_answer!(0);
    // part2_test!(0);
    // part2_answer!(0);
}
