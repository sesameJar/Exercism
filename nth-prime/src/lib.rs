pub fn nth(n: usize) -> u32 {
    let mut primes: Vec<u32> = vec![];
    let mut cnt = 2;
    while primes.len() != (n + 1) {
        if is_prime(&cnt) {
            println!("{}", cnt);
            primes.push(cnt);
        }
        cnt += 1;
    }
    primes[n]
}

fn is_prime(num: &u32) -> bool {
    println!("{}", &num);
    for i in 2..*num {
        if num % i == 0 && *num != i {
            return false;
        }
    }
    true
}
