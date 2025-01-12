use utils::*;

fn part1(input: &str) -> usize {
    let rgx = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    rgx.captures_iter(input)
        .filter_map(|m| {
            Some(
                m.get(1)?.as_str().parse::<usize>().ok()?
                    * m.get(2)?.as_str().parse::<usize>().ok()?,
            )
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let do_rgx = regex::Regex::new(r"do\(\)").unwrap();
    let dont_rgx = regex::Regex::new(r"don't\(\)").unwrap();
    let mut instructions = do_rgx.find_iter(input).collect::<Vec<_>>();
    instructions.extend(dont_rgx.find_iter(input));
    instructions.sort_by(|a, b| a.start().cmp(&b.start()));
    let rgx = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    rgx.captures_iter(input)
        .filter_map(|m| {
            let m_start = m.get(0)?.start();
            if let Some(i) = instructions.iter().rfind(|i| i.start() < m_start) {
                if i.as_str().starts_with("don'") {
                    return None;
                }
            };
            Some(
                m.get(1)?.as_str().parse::<usize>().ok()?
                    * m.get(2)?.as_str().parse::<usize>().ok()?,
            )
        })
        .sum()
}

fn main() {
    part1_test!(161);
    part1_answer!(175015740);
    test_part2(|| part2(include_str!("input/test2.txt")), 48);
    part2_answer!(112272912);
}
