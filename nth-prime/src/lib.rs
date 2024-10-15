pub fn nth(n: u32) -> u32 {
    let mut primes: Vec<u32> = Vec::new();
    let mut value: u32 = 2;
    while primes.len() <= n as usize {
        if is_prime(value) {
            primes.push(value);
        }
        value += 1;
    }
    primes[n as usize]
}

fn is_prime(n: u32) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    for i in (3..n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

