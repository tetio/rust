pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 2_u64.pow(s - 1),
        _ => panic!("Square out of range"),
    }
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
