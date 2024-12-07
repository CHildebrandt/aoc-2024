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

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Op {
    fn all_variants() -> &'static [Self] {
        &[Self::Add, Self::Mul, Self::Concat]
    }

    fn variants_p1() -> &'static [Self] {
        &[Self::Add, Self::Mul]
    }

    fn eval(&self, a: usize, b: usize) -> usize {
        match self {
            Self::Add => a + b,
            Self::Mul => a * b,
            Self::Concat => format!("{}{}", a, b).parse::<usize>().unwrap(),
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
    fn all_variants_from(nums: &Vec<usize>, variants: &'static [Op]) -> Vec<Self> {
        let mut exprs = vec![];
        for i in 0..variants.len().pow(nums.len() as u32) {
            let mut symbols = vec![];
            for (j, num) in nums.iter().enumerate() {
                symbols.push(Symbol::Num(*num));
                if j < nums.len() - 1 {
                    symbols.push(Symbol::Op(
                        variants[(i / variants.len().pow(j as u32)) % variants.len()],
                    ));
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
        if Expr::all_variants_from(nums, Op::variants_p1())
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
    let input = parse(input);
    input.iter().fold(0, |acc, (sum, nums)| {
        if Expr::all_variants_from(nums, Op::all_variants())
            .iter()
            .any(|expr| expr.eval() == *sum)
        {
            acc + sum
        } else {
            acc
        }
    })
}

fn main() {
    test_part1(|| part1(TEST), 3749);
    answer_part1(|| part1(INPUT), 975671981569);
    test_part2(|| part2(TEST), 11387);
    answer_part2(|| part2(INPUT), 223472064194845);
}
