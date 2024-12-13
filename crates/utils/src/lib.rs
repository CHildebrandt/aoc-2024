pub mod direction;
pub mod grid;
pub mod grid2;

pub fn split_double_newline(input: &str) -> Vec<&str> {
    let re = regex::Regex::new(r"\r?\n\r?\n").unwrap();
    re.split(input).collect()
}

fn answer(get: impl FnOnce() -> usize, actual: usize, is_test: bool, part: u8) {
    let time = std::time::Instant::now();
    assert_eq!(get(), actual);
    println!(
        "Part {} {}passed!",
        part,
        if is_test { "test " } else { "" }
    );
    println!("Elapsed: {:.2?}", time.elapsed());
}
pub fn test_part1(get: impl FnOnce() -> usize, actual: usize) {
    answer(get, actual, true, 1);
}
pub fn answer_part1(get: impl FnOnce() -> usize, actual: usize) {
    answer(get, actual, false, 1);
}
pub fn test_part2(get: impl FnOnce() -> usize, actual: usize) {
    answer(get, actual, true, 2);
}
pub fn answer_part2(get: impl FnOnce() -> usize, actual: usize) {
    answer(get, actual, false, 2);
}

#[macro_export]
macro_rules! part1_test {
    ($x:expr) => {{
        utils::test_part1(|| part1(&include_str!("./input/test.txt").trim()), $x);
    }};
}

#[macro_export]
macro_rules! part1_answer {
    ($x:expr) => {{
        utils::answer_part1(|| part1(&include_str!("./input/input.txt").trim()), $x);
    }};
}

#[macro_export]
macro_rules! part2_test {
    ($x:expr) => {{
        utils::test_part2(|| part2(&include_str!("./input/test.txt").trim()), $x);
    }};
}

#[macro_export]
macro_rules! part2_answer {
    ($x:expr) => {{
        utils::answer_part2(|| part2(&include_str!("./input/input.txt").trim()), $x);
    }};
}
