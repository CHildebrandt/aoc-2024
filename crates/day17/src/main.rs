use utils::*;

fn part1(input: &str) -> usize {
    let input = split_double_newline(input);
    let mut a = input[0]
        .lines()
        .nth(0)
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<usize>()
        .unwrap();
    let mut b = 0;
    let mut c = 0;
    let program = input[1]
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut output = Vec::new();
    let mut i_ptr = 0;
    while let Some((opcode, &operand)) = program.get(i_ptr).zip(program.get(i_ptr + 1)) {
        let combo = match operand {
            0..=3 => operand,
            4 => a,
            5 => b,
            6 => c,
            _ => 0, // Invalid
        };
        match opcode {
            0 => {
                a = (a as f32 / 2f32.powf(combo as f32)).floor() as usize;
            }
            1 => {
                b = b ^ operand;
            }
            2 => {
                b = combo % 8;
            }
            3 => {
                if a != 0 {
                    i_ptr = operand;
                    continue;
                }
            }
            4 => {
                b = b ^ c;
            }
            5 => {
                output.push(combo % 8);
            }
            6 => {
                b = (a as f32 / 2f32.powf(combo as f32)).floor() as usize;
            }
            7 => {
                c = (a as f32 / 2f32.powf(combo as f32)).floor() as usize;
            }
            _ => panic!("Invalid instruction"),
        }
        i_ptr += 2;
    }
    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("")
        .parse()
        .unwrap()
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    part1_test!(4635635210);
    part1_answer!(437153054);
    // part2_test!(0);
    // part2_answer!(0);
}
