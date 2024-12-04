enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn find(map: Vec<Vec<&char>>, direction: Direction, curr: (usize, usize), remaining: Vec<char>) {}

fn part1(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    input.iter().enumerate().fold(0, |acc, (y, curr)| {
        for (x, c) in curr.iter().enumerate() {
            match *c {
                'X' => {
                    if let (Some('M')) = (input.get(y).and_then(|row| row.get(x + 1))) {
                        println!("Found M at {}, {}", x, y);
                    }
                }
                'S' => {}
                _ => {}
            }
        }
        acc
    })
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    assert_eq!(part1(include_str!("./test")), 18);
    // assert_eq!(part1(include_str!("./input")), 0);
    // assert_eq!(part2(include_str!("./test")), 0);
    // assert_eq!(part2(include_str!("./input")), 0);
}
