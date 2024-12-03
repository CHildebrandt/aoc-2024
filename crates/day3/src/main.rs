fn part1(input: &str) -> usize {
    let rgx = regex::Regex::new(r"mul\(\d+,\d+\)").unwrap();
    rgx.find_iter(input)
        .filter_map(|m| {
            let mut nums = m
                .as_str()
                .split("mul(")
                .nth(1)?
                .split(")")
                .nth(0)?
                .split(",");
            let a = nums.next()?.parse::<usize>().ok()?;
            let b = nums.next()?.parse::<usize>().ok()?;
            Some(a * b)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let dos = regex::Regex::new(r"do\(\)").unwrap();
    let donts = regex::Regex::new(r"don\'t\(\)").unwrap();
    let mut instructions = dos
        .find_iter(input)
        .zip(donts.find_iter(input))
        .flat_map(|(a, b)| vec![a, b])
        .collect::<Vec<_>>();
    instructions.sort_by(|a, b| a.start().cmp(&b.start()));
    let rgx = regex::Regex::new(r"mul\(\d+,\d+\)").unwrap();
    rgx.find_iter(input)
        .filter_map(|m| {
            match instructions.iter().rfind(|i| i.start() < m.start()) {
                Some(i) => {
                    if i.as_str().starts_with("don'") {
                        return None;
                    }
                }
                None => (),
            };
            let mut nums = m
                .as_str()
                .split("mul(")
                .nth(1)?
                .split(")")
                .nth(0)?
                .split(",");
            let a = nums.next()?.parse::<usize>().ok()?;
            let b = nums.next()?.parse::<usize>().ok()?;
            Some(a * b)
        })
        .sum()
}

fn main() {
    assert_eq!(part1(include_str!("./test")), 161);
    assert_eq!(part1(include_str!("./input")), 175015740);
    assert_eq!(part2(include_str!("./test2")), 48);
    assert_eq!(part2(include_str!("./input")), 0);
}
