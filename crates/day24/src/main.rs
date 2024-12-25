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

impl Gate {
    fn has_input(&self, name: &str) -> bool {
        self.input_a == name || self.input_b == name
    }

    fn any_connection_starts_with(&self, name: &[char]) -> bool {
        self.input_a.starts_with(name)
            || self.input_b.starts_with(name)
            || self.output.starts_with(name)
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
        .collect_vec();
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

fn part2(input: &str) -> String {
    let (_, gates) = split_double_newline_once(input);
    let gates = gates.lines().map(Gate::from).collect_vec();
    let max_z = gates
        .iter()
        .filter_map(|gate| gate.output.starts_with("z").then_some(gate.output.clone()))
        .max()
        .unwrap();
    gates
        .iter()
        .filter(|gate| {
            if gate.output.starts_with('z') && gate.output != max_z && gate.logic != Logic::XOr {
                true
            } else {
                match gate.logic {
                    Logic::And => {
                        !gate.has_input("x00")
                            && gates.iter().any(|checked| {
                                checked.has_input(&gate.output) && checked.logic != Logic::Or
                            })
                    }
                    Logic::Or => false,
                    Logic::XOr => {
                        !gate.any_connection_starts_with(&['x', 'y', 'z'])
                            || gates.iter().any(|checked| {
                                checked.has_input(&gate.output) && checked.logic == Logic::Or
                            })
                    }
                }
            }
        })
        .map(|gate| gate.output.clone())
        .sorted()
        .join(",")
}

fn main() {
    part1_test!(4);
    test_part1(|| part1(include_str!("./input/test2.txt")), 2024);
    part1_answer!(55114892239566);
    part2_answer!("cdj,dhm,gfm,mrb,qjd,z08,z16,z32".into());
}
