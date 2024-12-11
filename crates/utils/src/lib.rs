pub mod direction;
pub mod grid;

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
    ($a:expr, $b:expr) => {{
        utils::test_part1(|| part1(&$a.trim()), $b);
    }};
}

#[macro_export]
macro_rules! part1_answer {
    ($a:expr, $b:expr) => {{
        utils::answer_part1(|| part1(&$a.trim()), $b);
    }};
}

#[macro_export]
macro_rules! part2_test {
    ($a:expr, $b:expr) => {{
        utils::test_part2(|| part1(&$a.trim()), $b);
    }};
}

#[macro_export]
macro_rules! part2_answer {
    ($a:expr, $b:expr) => {{
        utils::answer_part2(|| part1(&$a.trim()), $b);
    }};
}
