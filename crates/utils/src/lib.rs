use std::fmt::Debug;

pub mod direction;
pub mod grid;
// pub mod grid2;

pub fn split_double_newline(input: &str) -> Vec<&str> {
    let re = regex::Regex::new(r"\r?\n\r?\n").unwrap();
    re.split(input).collect()
}

pub fn split_double_newline_once(input: &str) -> (&str, &str) {
    let v = split_double_newline(input);
    (v[0], v[1])
}

pub fn extract_last_digit(num: usize) -> usize {
    num.to_string()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap() as usize
}

fn answer<T: PartialEq + Debug>(get: impl FnOnce() -> T, actual: T, is_test: bool, part: u8) {
    let time = std::time::Instant::now();
    assert_eq!(get(), actual);
    println!(
        "Part {} {}passed!",
        part,
        if is_test { "test " } else { "" }
    );
    println!("Elapsed: {:.2?}", time.elapsed());
}
pub fn test_part1<T: PartialEq + Debug>(get: impl FnOnce() -> T, actual: T) {
    answer(get, actual, true, 1);
}
pub fn answer_part1<T: PartialEq + Debug>(get: impl FnOnce() -> T, actual: T) {
    answer(get, actual, false, 1);
}
pub fn test_part2<T: PartialEq + Debug>(get: impl FnOnce() -> T, actual: T) {
    answer(get, actual, true, 2);
}
pub fn answer_part2<T: PartialEq + Debug>(get: impl FnOnce() -> T, actual: T) {
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
