pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    } else if s == 1 {
        return 1;
    }

    square(s - 1) * 2
}

pub fn total() -> u64 {
    (1..=64).map(|x| square(x)).sum()
}
