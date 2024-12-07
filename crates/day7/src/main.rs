use utils::*;

const TEST: &str = include_str!("./input/test.txt");
const INPUT: &str = include_str!("./input/input.txt");

fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(l, r)| {
                    (
                        l.parse::<usize>().unwrap(),
                        r.split(" ")
                            .map(|x| x.parse::<usize>().unwrap())
                            .collect::<Vec<_>>(),
                    )
                })
                .unwrap()
        })
        .collect::<Vec<_>>()
}

enum Op {
    Add,
    Mul,
}

impl Op {
    fn variants() -> &'static [Self] {
        &[Self::Add, Self::Mul]
    }

    fn eval(&self, a: usize, b: usize) -> usize {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
        }
    }
}

enum Symbol {
    Num(usize),
    Op(Op),
}

struct Expr {
    symbols: Vec<Symbol>,
}

impl Expr {
    fn all_variants_from(nums: &Vec<usize>) -> Vec<Self> {
        let mut exprs = vec![];
        for i in 0..Op::variants().len().pow(nums.len() as u32) {
            let mut symbols = vec![];
            for (j, num) in nums.iter().enumerate() {
                symbols.push(Symbol::Num(*num));
                if j < nums.len() - 1 {
                    let op = match Op::variants()[(i / Op::variants().len().pow(j as u32)) % 2] {
                        Op::Add => Symbol::Op(Op::Add),
                        Op::Mul => Symbol::Op(Op::Mul),
                    };
                    symbols.push(op);
                }
            }
            exprs.push(Self { symbols });
        }
        exprs
    }

    fn eval(&self) -> usize {
        let mut sum = 0;
        for (i, symbol) in self.symbols.iter().enumerate() {
            match symbol {
                Symbol::Num(n) => {
                    if i == 0 {
                        sum = *n;
                    }
                }
                Symbol::Op(op) => {
                    let next = match self.symbols.get(i + 1) {
                        Some(Symbol::Num(n)) => n,
                        _ => panic!("Expected number"),
                    };
                    sum = op.eval(sum, *next);
                }
            }
        }
        sum
    }
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    input.iter().fold(0, |acc, (sum, nums)| {
        if Expr::all_variants_from(nums)
            .iter()
            .any(|expr| expr.eval() == *sum)
        {
            acc + sum
        } else {
            acc
        }
    })
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    test_part1(|| part1(TEST), 3749);
    answer_part1(|| part1(INPUT), 975671981569);
    // test_part2(|| part1(TEST), 0);
    // answer_part2(|| part1(INPUT), 0);
}
