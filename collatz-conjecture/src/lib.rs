pub fn collatz(n: u64) -> Option<u64> {
    match n == 0 {
        true => None,
        false => Some(do_collatz(n, 0))
    }
}

fn do_collatz(n: u64, mut steps: u64) -> u64 {
    if n == 1 {
        steps
    } else {
        steps += 1;
        match n % 2 == 0 {
            true => do_collatz(n/2, steps),
            false => do_collatz(3*n + 1, steps),
        }
    }
}
