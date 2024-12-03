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
    assert_eq!(part1(include_str!("./test")), 161);
    assert_eq!(part1(include_str!("./input")), 175015740);
    assert_eq!(part2(include_str!("./test2")), 48);
    assert_eq!(part2(include_str!("./input")), 112272912);
}
