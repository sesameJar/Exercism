pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .filter(|x| {
            let mut v = factors.to_owned();
            v.retain(|&x| x != 0);
            for i in v {
                if x % i == 0 {
                    return true;
                }
            }
            false
        })
        .sum::<u32>()
}
