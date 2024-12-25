use std::collections::HashMap;

use itertools::Itertools;
use utils::*;

struct Wire {
    name: String,
    is_on: bool,
}

impl From<&str> for Wire {
    fn from(s: &str) -> Self {
        let (name, is_on) = s.split_once(": ").unwrap();
        Self {
            name: name.to_string(),
            is_on: is_on == "1",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Logic {
    And,
    Or,
    XOr,
}

impl From<&str> for Logic {
    fn from(s: &str) -> Self {
        match s {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::XOr,
            _ => unreachable!(),
        }
    }
}

impl Logic {
    fn eval(&self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a && b,
            Self::Or => a || b,
            Self::XOr => a ^ b,
        }
    }
}

#[derive(Debug)]
struct Gate {
    input_a: String,
    logic: Logic,
    input_b: String,
    output: String,
}

impl From<&str> for Gate {
    fn from(s: &str) -> Self {
        let mut split = s.split_whitespace().filter(|s| *s != "->");
        Self {
            input_a: split.next().unwrap().to_string(),
            logic: split.next().unwrap().into(),
            input_b: split.next().unwrap().to_string(),
            output: split.next().unwrap().to_string(),
        }
    }
}

fn part1(input: &str) -> usize {
    let (wires, gates) = split_double_newline_once(input);
    let mut wires = wires
        .lines()
        .map(Wire::from)
        .map(|wire| (wire.name, wire.is_on))
        .collect::<HashMap<_, _>>();
    let gates = gates.lines().map(Gate::from).collect_vec();
    let z_gates = gates
        .iter()
        .filter_map(|gate| gate.output.starts_with("z").then_some(gate.output.clone()))
        .collect::<Vec<_>>();
    loop {
        for gate in &gates {
            if let Some((&a, &b)) = wires.get(&gate.input_a).zip(wires.get(&gate.input_b)) {
                wires.insert(gate.output.clone(), gate.logic.eval(a, b));
            }
        }
        if z_gates.iter().all(|name| wires.get(name).is_some()) {
            break;
        }
    }
    let bin = wires
        .iter()
        .filter(|(name, _)| name.starts_with("z"))
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .rev()
        .map(|(_, is_on)| if *is_on { "1" } else { "0" })
        .collect::<String>();
    usize::from_str_radix(&bin, 2).unwrap()
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    part1_test!(4);
    test_part1(|| part1(include_str!("./input/test2.txt")), 2024);
    part1_answer!(55114892239566);
    // part2_test!(0);
    // part2_answer!(0);
}
