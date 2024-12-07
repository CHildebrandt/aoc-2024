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
