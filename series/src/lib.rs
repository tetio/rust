pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits.chars()
    .collect::<Vec<char>>()
    .windows(len)
    .map(String::from_iter)
    .collect()
}