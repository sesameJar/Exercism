fn is_prime(n: u64) -> bool {
    !(2..(n as f64).sqrt() as u64).any(|x| n % x == 0)
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut factors: Vec<u64> = vec![];
    while n != factors.iter().product() {
        'inner: for i in 2..=num {
            if is_prime(i) && num % i == 0 {
                factors.push(i);
                num /= i;
                println!("{}", num);
                break 'inner;
            }
        }
    }
    factors
}
