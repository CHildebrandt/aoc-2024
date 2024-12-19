use utils::*;

#[derive(Clone)]
struct Program {
    instructions: Vec<usize>,
    a: usize,
    b: usize,
    c: usize,
    ptr: usize,
    output: Vec<usize>,
}

impl Program {
    fn from_str(input: &str) -> Self {
        let (a, b) = split_double_newline_once(input);
        let a = a
            .lines()
            .nth(0)
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let program = b
            .split_once(": ")
            .unwrap()
            .1
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Self {
            instructions: program,
            a,
            b: 0,
            c: 0,
            ptr: 0,
            output: Vec::new(),
        }
    }

    fn reset(&mut self, a: usize) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.ptr = 0;
        self.output.clear();
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn a_div(&self, operand: usize) -> usize {
        (self.a as f64 / 2f64.powf(self.combo(operand) as f64)) as usize
    }

    fn run(&mut self) {
        while let Some((opcode, &operand)) = self
            .instructions
            .get(self.ptr)
            .zip(self.instructions.get(self.ptr + 1))
        {
            match opcode {
                0 => {
                    self.a = self.a_div(operand);
                }
                1 => {
                    self.b ^= operand;
                }
                2 => {
                    self.b = self.combo(operand) % 8;
                }
                3 => {
                    if self.a != 0 {
                        self.ptr = operand;
                        continue;
                    }
                }
                4 => {
                    self.b ^= self.c;
                }
                5 => {
                    self.output.push(self.combo(operand) % 8);
                }
                6 => {
                    self.b = self.a_div(operand);
                }
                7 => {
                    self.c = self.a_div(operand);
                }
                _ => unreachable!(),
            }
            self.ptr += 2;
        }
    }

    fn get_best_instruction_clone(&mut self, cursor: usize, curr_a: usize) -> Option<usize> {
        for curr_a in (0..8).map(|x| curr_a * 8 + x) {
            self.reset(curr_a);
            self.run();
            if self.output == self.instructions[cursor..] {
                if cursor == 0 {
                    return Some(curr_a);
                }
                if let Some(ret) = self.clone().get_best_instruction_clone(cursor - 1, curr_a) {
                    return Some(ret);
                }
            }
        }
        None
    }
}

fn part1(input: &str) -> usize {
    let mut program = Program::from_str(input);
    program.run();
    program
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn part2(input: &str) -> usize {
    let mut program = Program::from_str(input);
    program
        .get_best_instruction_clone(program.instructions.len() - 1, 0)
        .unwrap()
}

fn main() {
    part1_test!(4635635210);
    part1_answer!(437153054);
    test_part2(|| part2(include_str!("./input/test2.txt")), 117440);
    part2_answer!(190384615275535);
}
